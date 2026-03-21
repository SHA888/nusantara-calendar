# Architecture — `nusantara-calendar`

## Table of Contents

1. [Design Philosophy](#1-design-philosophy)
2. [JDN Pivot Pattern](#2-jdn-pivot-pattern)
3. [Crate Dependency Graph](#3-crate-dependency-graph)
4. [Core Trait Contract](#4-core-trait-contract)
5. [`no_std` Strategy](#5-no_std-strategy)
6. [Feature Flag System](#6-feature-flag-system)
7. [Data Representation Policy](#7-data-representation-policy)
8. [The `stub!()` Contract](#8-the-stub-contract)
9. [Observation-Dependent Calendars](#9-observation-dependent-calendars)
10. [WASM Compilation](#10-wasm-compilation)
11. [`dewasa-engine` Aggregation Model](#11-dewasa-engine-aggregation-model)
12. [Dependency Decisions & Licenses](#12-dependency-decisions--licenses)
13. [Error Taxonomy](#13-error-taxonomy)
14. [Versioning & Stability Guarantees](#14-versioning--stability-guarantees)

---

## 1. Design Philosophy

Three non-negotiable properties, in priority order:

1. **Correctness with citation.** Every algorithm must trace to a citable, verifiable
   source. The bar is: *could a reviewer independently verify this against the cited
   reference without running the code?* If no, the implementation is a stub.

2. **Interoperability through a universal pivot.** Julian Day Number (JDN) is the
   single exchange format between all calendar systems. No calendar crate imports
   another calendar crate directly — they only share `calendar-core`.

3. **Composability at the type level.** The `CalendarDate` trait is the only required
   interface. A `dewasa-engine` instance can hold any combination of calendar
   representations without coupling their implementations.

---

## 2. JDN Pivot Pattern

```
Gregorian ──────┐
Javanese ───────┤
Balinese ───────┤      ┌─────────────────┐      ┌──────────────┐
Hijriyah ───────┼─────►│  JDN (i64)      │◄────►│ Target crate │
Chinese ────────┤      └─────────────────┘      └──────────────┘
Batak ──────────┤
Sunda ──────────┘
```

**Why JDN and not Unix timestamps or chrono types?**

- JDN is integer — no floating-point precision loss over multi-century spans.
- JDN predates the Unix epoch by millennia; the Balinese Saka epoch (78 CE) and the
  Sultan Agung epoch (1633 CE) are both straightforwardly representable.
- JDN is `no_std` friendly — it is a plain `i64` with no runtime dependencies.
- JDN is the canonical interop format in Dershowitz-Reingold (the authoritative
  reference for the algorithms used here), making algorithm translation direct.

**JDN functions** (in `calendar-core`, `no_std`):

```rust
// Meeus, Astronomical Algorithms, Ch. 7 — proleptic Gregorian
pub const fn gregorian_to_jdn(y: i32, m: u8, d: u8) -> i64 { ... }
pub const fn jdn_to_gregorian(jdn: i64) -> (i32, u8, u8)   { ... }
```

Both are `const fn` — usable in `const` contexts and in `#[no_std]` targets.

**Verification anchor:** `gregorian_to_jdn(1582, 10, 15) == 2299161` (Gregorian reform date).

---

## 3. Crate Dependency Graph

```
                         ┌──────────────────┐
                         │   calendar-core   │
                         │  (no_std + alloc) │
                         └────────┬─────────┘
              ┌──────────┬────────┼────────────────────────────────┐
              │          │        │            │                    │
    ┌─────────▼──┐  ┌────▼───┐  ┌▼───────┐  ┌▼──────────────┐  ┌─▼──────────────┐
    │ balinese-  │  │  jawa  │  │hijriyah│  │chinese-        │  │  batak/sunda/  │
    │ calendar   │  │        │  │        │  │nusantara       │  │  tengger/bugis │
    │ (no_std)   │  │(no_std)│  │(no_std)│  │(std, nongli)   │  │  sasak/dayak/  │
    └─────────┬──┘  └────┬───┘  └──┬─────┘  └───┬────────────┘  │  toraja/       │
              │          │         │             │               │  minangkabau   │
              └──────────┴────┬────┴─────────────┘               └───┬────────────┘
                              │                                       │
                    ┌─────────▼───────────────────────────────────────▼──┐
                    │                  dewasa-engine                      │
                    │          (std, HashMap aggregation)                 │
                    └────────────────────────────────────────────────────┘
```

**Rule:** No calendar crate may import another calendar crate. All shared state flows
through `calendar-core` types only. This keeps crates independently publishable and
prevents circular dependency chains.

---

## 4. Core Trait Contract

### `CalendarDate`

```rust
pub trait CalendarDate: Sized + Clone + PartialEq {
    fn from_jdn(jdn: i64) -> Result<Self, CalendarError>;
    fn to_jdn(&self) -> i64;

    // Default impls — do not override unless the calendar's natural representation
    // requires a non-standard Gregorian ↔ JDN mapping.
    fn from_gregorian(y: i32, m: u8, d: u8) -> Result<Self, CalendarError>;
    fn to_gregorian(&self) -> (i32, u8, u8);
}
```

**Invariant:** `Self::from_jdn(x.to_jdn()) == Ok(x)` for all valid `x`. Tests must
verify this round-trip for at least 100 diverse dates spanning the full supported range
of each crate.

### `CalendarMetadata`

```rust
pub trait CalendarMetadata {
    fn calendar_name() -> &'static str;
    fn ethnic_group() -> &'static str;
    fn region() -> &'static str;
    fn epoch_jdn() -> i64;
    fn reference_sources() -> &'static [&'static str];
}
```

`reference_sources()` must return at least one citable source. Returning an empty slice
is a compile-time warning (enforced via a `const` assertion in tests).

### `HasAuspiciousness`

Optional trait. Implement only when the calendar system has documented auspiciousness
rules that can be algorithmically derived. Do **not** implement by guessing.

---

## 5. `no_std` Strategy

| Crate | Policy | Justification |
|---|---|---|
| `calendar-core` | `no_std + alloc` | All types are plain data; no OS interaction |
| `balinese-calendar` | `no_std + alloc` | Pure modular arithmetic on `const` tables |
| `jawa` | `no_std + alloc` | Pure modular arithmetic on `const` tables |
| `hijriyah` (Option A) | `no_std + alloc` | Tabular algorithm from D-R Ch. 6; no float |
| `chinese-nusantara` | `std` required | `nongli` → `chrono` → `std` |
| `batak` (tabular) | `no_std + alloc` | Tabular Porhalaan; lookup tables as `const` |
| `batak` (astronomical) | `std` required | `libm` or `std::f64` for heliacal angle calc |
| `sunda`, `tengger` | `no_std + alloc` | Saka arithmetic |
| `bugis`, `sasak`, `dayak`, `toraja`, `minangkabau` | `no_std + alloc` | Cycle arithmetic; stubs for unverified parts |
| `dewasa-engine` | `std` required | `HashMap<Activity, CrossCalendarVerdict>` |

**Implementation pattern for crates that need both:**

```toml
# Cargo.toml
[features]
default = ["std"]
std = []
astronomical = ["std", "libm"]

[dependencies]
libm = { version = "0.2", optional = true }
```

```rust
// lib.rs
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
```

**`alloc` usage is permitted everywhere.** `String` (for error messages), `Vec` (for
dynamic verdict lists in `dewasa-engine`), and `Box<dyn Error>` are all fine. The
constraint is *no `std`*, not *no heap*.

---

## 6. Feature Flag System

Every crate in the workspace uses the same four flags for consistency:

```toml
[features]
default = []        # no features on by default; let consumers opt in
std          = []   # enables std-dependent impls
serde        = ["dep:serde"]
wasm         = ["dep:wasm-bindgen"]
astronomical = ["std", "dep:libm"]   # only in observation-dependent crates
```

**Flag semantics:**

- `std`: unlocks `std::error::Error` impl on `CalendarError`, `HashMap` in
  `dewasa-engine`, and `std::fmt::Display` improvements.
- `serde`: adds `#[derive(Serialize, Deserialize)]` to all public structs and enums.
  Gated because serde adds compile time; not every consumer needs it.
- `wasm`: adds `#[wasm_bindgen]` exports. Requires `wasm-bindgen` in the toolchain.
  Must not break non-WASM builds when absent.
- `astronomical`: enables floating-point heliacal rise/set calculations. This is
  the only feature that changes computation semantics (vs. just adding traits).

---

## 7. Data Representation Policy

**All static lookup tables are `const` arrays.** This means:

```rust
// ✅ correct
pub const WUKU_NAMES: [&str; 30] = [
    "Sinta", "Landep", "Ukir", /* ... */
];

// ❌ forbidden
pub static WUKU_NAMES: Vec<&str> = vec![ /* ... */ ]; // heap at runtime
pub fn wuku_name(i: usize) -> &'static str { /* match */ } // fine, but prefer const array
```

Rationale: `const` arrays are zero-cost, WASM-safe, and verifiable against source
tables by direct inspection. Dynamic allocation for lookup data indicates a design
error.

**Integer types for cycle positions:**

| Cycle | Type | Rationale |
|---|---|---|
| JDN | `i64` | Signed: pre-epoch dates are negative JDN |
| AJ year | `u32` | Positive-only; 32 bits sufficient for centuries |
| Wuku position | `u8` | 0–29; fits in a byte |
| Pasaran position | `u8` | 0–4 |
| Saptawara position | `u8` | 0–6 |
| Windu position | `u8` | 0–7 (also `WinduYear` enum) |
| Hijri year | `i32` | Signed: year 1 AH is positive, proleptic years negative |

---

## 8. The `stub!()` Contract

```rust
#[macro_export]
macro_rules! stub {
    ($msg:literal) => {
        return Err($crate::CalendarError::NotImplemented($msg.to_string()))
    };
}
```

**When to use `stub!()`:**

- The calendar system is known to exist but no published algorithm is available.
- A sub-group variant diverges from the core system in a way not yet documented.
- A supra-cycle name (e.g., Batak supra-windu groups, Sunda Wiwitan ceremonial calendar)
  lacks a citable algorithm.

**`stub!()` documentation requirement:**

Every `stub!()` call must have a `/// # Sources` comment directly above it listing:
1. What is known about the feature.
2. Why it cannot be algorithmically implemented yet.
3. The primary source that documents its existence.

```rust
/// # Sources
/// - Sunda Wiwitan ceremonial calendar: oral tradition of Baduy (Kanekes) community.
///   No published algorithm. Existence documented in: Garna (1993), *Masyarakat Baduy*.
pub fn sunda_wiwitan_date(&self) -> Result<SundaWiwitanDay, CalendarError> {
    stub!("Sunda Wiwitan: oral tradition, no published algorithm. See Garna (1993).")
}
```

**`stub!()` is not a TODO comment.** It is a first-class API response. Consumers can
pattern-match on `CalendarError::NotImplemented` and handle gracefully.

---

## 9. Observation-Dependent Calendars

Three crates have calendar cycles that depend on heliacal astronomical events that
cannot be computed by pure tabular arithmetic:

| Crate | Observation event | Observer latitude |
|---|---|---|
| `batak` | Orion (Waluku) / Scorpius first rise | Lake Toba, ~2.6°N |
| `sasak` | Pleiades (Bintang Rowot) first rise | Lombok, ~8.6°S |
| `bugis`, `minangkabau` | Pleiades (Bintang Kartika) visibility | Sulawesi ~4°S / Sumatra ~0° |

**Dual-mode API pattern (mandatory for all observation-dependent crates):**

```rust
impl BatakDay {
    /// Tabular fallback — deterministic, no_std, ignores heliacal drift.
    /// Use when you need a consistent result regardless of astronomical accuracy.
    pub fn from_jdn_tabular(jdn: i64) -> Result<Self, CalendarError> { ... }

    /// Astronomical computation — requires `astronomical` feature.
    /// Uses observer latitude/longitude to compute actual Orion/Scorpius visibility.
    #[cfg(feature = "astronomical")]
    pub fn from_jdn_astronomical(
        jdn: i64,
        lat: f64,
        lon: f64,
    ) -> Result<Self, CalendarError> { ... }
}
```

The `CalendarDate::from_jdn()` blanket impl calls `from_jdn_tabular()`. Astronomical
results are opt-in. This preserves the `no_std` guarantee on the default feature set.

---

## 10. WASM Compilation

All `no_std + alloc` crates must compile cleanly to `wasm32-unknown-unknown`:

```sh
# Verified in CI for each crate
cargo build --target wasm32-unknown-unknown -p calendar-core --no-default-features
cargo build --target wasm32-unknown-unknown -p jawa --no-default-features
cargo build --target wasm32-unknown-unknown -p balinese-calendar --no-default-features
cargo build --target wasm32-unknown-unknown -p hijriyah --no-default-features
```

`chinese-nusantara` and `dewasa-engine` do **not** have a WASM build target. This is
expected and is documented in their `Cargo.toml` with:

```toml
[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]  # WASM not supported for this crate
```

**WASM `wasm_bindgen` exports** (when `wasm` feature is enabled) expose a flat,
date-string-based API suitable for JavaScript consumers of `dedauh.id`:

```rust
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn balinese_from_gregorian(y: i32, m: u8, d: u8) -> Result<JsValue, JsValue> {
    BalineseDay::from_gregorian(y, m, d)
        .map(|d| serde_wasm_bindgen::to_value(&d).unwrap())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
```

---

## 11. `dewasa-engine` Aggregation Model

`dewasa-engine` is the only crate that imports more than one calendar crate. Its
aggregation model:

```
Input: Gregorian date (y, m, d)
       ↓
       gregorian_to_jdn → JDN
       ↓
   ┌───┴──────────────────────────────────────────┐
   │  Parallel conversion (independent, fallible) │
   │  balinese::from_jdn()                        │
   │  jawa::from_jdn()                            │
   │  hijriyah::from_jdn()                        │
   │  chinese_nusantara::from_jdn()               │
   │  batak::from_jdn_tabular()  [if feature]     │
   │  ...                                         │
   └───┬──────────────────────────────────────────┘
       ↓
   NusantaraDay { gregorian, jdn, balinese: Option<_>, ... }
       ↓
   CrossCalendarVerdict computation per Activity
   - Collect auspiciousness per calendar
   - Detect conflicts (auspicious in Bali, inauspicious in Jawa)
   - Overall = most conservative non-Unknown level
       ↓
Output: NusantaraDay with HashMap<Activity, CrossCalendarVerdict>
```

**Conflict definition:** A `CalendarConflict` is recorded when two or more calendars
return `AuspiciousnessLevel` values that are on opposite sides of `Neutral`
(i.e., one `Auspicious`/`HighlyAuspicious` and another `Inauspicious`/`Forbidden`).

**Consensus rule (default):** `overall` = the most cautious level across all calendars
that returned a non-`Unknown` result. `Unknown` is excluded from consensus — it does
not drag an otherwise `HighlyAuspicious` day down to `Unknown`.

---

## 12. Dependency Decisions & Licenses

| Dependency | Version | License | Used by | Decision |
|---|---|---|---|---|
| `thiserror` | 2.0.18 | MIT OR Apache-2.0 | `calendar-core` | Standard error derive |
| `nongli` | 0.4.1 | MIT | `chinese-nusantara` | Only Rust Chinese lunisolar crate with JDN-compatible API |
| `chrono` | 0.4.44 | MIT OR Apache-2.0 | `chinese-nusantara` (via nongli) | Transitive; `std` feature only |
| `serde` | 1.0.228 | MIT OR Apache-2.0 | all (optional `serde` feature) | De-facto serialization standard |
| `wasm-bindgen` | 0.2.114 | MIT OR Apache-2.0 | all (optional `wasm` feature) | Required for WASM JS interop |
| `libm` | 0.2.16 | MIT | `batak`, `sasak`, `bugis`, `minangkabau` (optional `astronomical`) | `no_std`-compatible float math |

**Explicitly excluded:**

- `misykat` (GPL-3.0-only) — copyleft; would force GPL-3.0-only terms on all dependents. Hijri arithmetic
  reimplemented independently from Dershowitz-Reingold Ch. 6 and Meeus Ch. 9 to keep the workspace permissive.
- `tanggalan` (bect/tanggalan) — does not exist on crates.io as of March 2026.
- Any crate providing astrology, prayer times, or timezone conversion — out of scope.

---

## 13. Error Taxonomy

```rust
pub enum CalendarError {
    OutOfRange(String),     // Date outside the crate's supported year range
    NotImplemented(String), // Algorithm known but not yet coded; stub!() target
    Ambiguous(String),      // Source data conflict — algorithm has multiple valid readings
}
```

**`OutOfRange` usage:** Each crate documents its supported year range. E.g., `jawa`
supports AJ 1555–2474 (Gregorian 1633–2169, spanning the current and next kurup).
Dates outside this range return `OutOfRange`, not a silently wrong result.

**`Ambiguous` usage:** Reserved for cases where two equally credible primary sources
give different rules for the same date element. The error message must cite both sources
and describe the divergence. This is not a stub — the algorithm exists but has
irreconcilable variant readings.

---

## 14. Versioning & Stability Guarantees

Follows [Semantic Versioning 2.0.0](https://semver.org/).

| Version series | Crates included | API stability |
|---|---|---|
| 0.x | All crates pre-1.0 | Breaking changes allowed between minors |
| 1.0.0 | All crates (post dewasa-engine stabilization) | `CalendarDate`, `CalendarMetadata`, `CalendarError` are stable |
| 1.x | All crates | Additive only: new calendar crates, new `Activity` variants (non-exhaustive), stub → impl upgrades |

**`#[non_exhaustive]`** is applied to `Activity` and `AuspiciousnessLevel`. Consumers
must handle `_` arms. This allows adding new activities without a semver break.

**Stub-to-implementation upgrades** (e.g., `stub!()` → real algorithm) are **not**
considered breaking changes, even though they change return values. The previous return
was `Err(NotImplemented)`. Any consumer that was pattern-matching on that and treating
it as "OK, no data" will now receive `Ok(data)` instead — this is strictly additive.
Document these upgrades in `CHANGELOG.md` under `### Added`.
