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
   single exchange format between all calendar systems. No calendar module imports
   another calendar module directly — they only share `calendar-core`.

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
// gregorian_to_jdn: Meeus, Astronomical Algorithms, Ch. 7 — proleptic Gregorian
// jdn_to_gregorian: Fliegel & van Flandern (1968), USNO algorithm
pub fn gregorian_to_jdn(year: i32, month: u8, day: u8) -> JDN { ... }
pub fn jdn_to_gregorian(jdn: JDN) -> (i32, u8, u8)            { ... }
```

Note: these are regular functions, not `const fn`. `jdn_to_gregorian` performs a
range-validity assertion that is not stable in `const` context.

**Verification anchor:** `gregorian_to_jdn(1582, 10, 15) == 2299161` (Gregorian reform date).
**Sultan Agung anchor:** `gregorian_to_jdn(1633, 7, 8) == 2317690`.

---

## 3. Crate Dependency Graph

```
                    ┌──────────────────────────┐
                    │       calendar-core       │
                    │  v0.1.0 · published       │
                    │  no_std + alloc           │
                    └────────────┬─────────────┘
                                 │
    ┌────────────────────────────┼────────────────────────────────────┐
    │                            │                                    │
    │  balinese-calendar v0.2.0  │                                    │
    │  (external, separate repo) │                                    │
    │  crates.io/crates/         │                                    │
    │  balinese-calendar         │                                    │
    └────────────┬───────────────┘                                    │
                 │                                                    │
    ┌────────────▼────────────────────────────────────────────────────▼──┐
    │                   nusantara-calendar v0.1.0                        │
    │               (feature-gated modules, not yet published)           │
    │                                                                    │
    │  src/balinese/     ← wraps balinese-calendar external dep          │
    │  src/jawa/         ← stub                                          │
    │  src/hijriyah/     ← stub                                          │
    │  src/batak/        ← stub                                          │
    │  src/sunda/        ← stub                                          │
    │  src/tengger/      ← stub                                          │
    │  src/bugis/        ← stub                                          │
    │  src/sasak/        ← stub                                          │
    │  src/dayak/        ← stub                                          │
    │  src/toraja/       ← stub                                          │
    │  src/minangkabau/  ← stub                                          │
    │  src/chinese_nusantara/ ← stub (will wrap nongli)                 │
    │  src/dewasa_engine/ ← stub (std-only, HashMap aggregation)        │
    └────────────────────────────────────────────────────────────────────┘
```

**Rule:** No calendar module may import another calendar module. All shared state flows
through `calendar-core` types only. This keeps modules independently testable and
prevents circular dependency chains.

**External vs internal distinction:**
- `calendar-core` — standalone published crate, no_std, no calendar logic
- `balinese-calendar` — standalone published crate, separate repository
- All other calendar systems — feature-gated modules within `nusantara-calendar`
- `nusantara-calendar` is the single publishable umbrella crate

---

## 4. Core Trait Contract

### `CalendarDate`

```rust
pub trait CalendarDate: Clone + PartialEq + Eq + core::fmt::Debug {
    fn from_jdn(jdn: JDN) -> Result<Self, CalendarError>
    where
        Self: Sized;

    fn to_jdn(&self) -> JDN;

    fn calendar_name() -> &'static str;

    fn validate_range(&self) -> Result<(), CalendarError>;

    // Default impls — do not override unless the calendar's natural representation
    // requires a non-standard Gregorian ↔ JDN mapping.
    fn from_gregorian(year: i32, month: u8, day: u8) -> Result<Self, CalendarError>
    where
        Self: Sized,
    {
        let jdn = gregorian_to_jdn(year, month, day);
        Self::from_jdn(jdn)
    }

    fn to_gregorian(&self) -> (i32, u8, u8) {
        jdn_to_gregorian(self.to_jdn())
    }
}
```

**Invariant:** `Self::from_jdn(x.to_jdn()) == Ok(x)` for all valid `x`. Tests must
verify this round-trip for at least 100 diverse dates spanning the full supported range
of each module.

### `CalendarMetadata`

```rust
pub trait CalendarMetadata {
    fn epoch() -> JDN;
    fn cycle_length() -> Option<CycleYear> { None }
    fn description() -> &'static str;
    fn cultural_origin() -> &'static str;
}
```

`description()` and `cultural_origin()` must return non-empty strings. Returning an
empty `""` is a documentation failure and must be caught in tests via a non-empty
assertion.

### `HasAuspiciousness`

Optional trait. Implement only when the calendar system has documented auspiciousness
rules that can be algorithmically derived. Do **not** implement by guessing.

```rust
pub trait HasAuspiciousness {
    type Activity;
    type AuspiciousnessLevel;

    fn auspiciousness_for(&self, activity: &Self::Activity) -> Self::AuspiciousnessLevel;
    fn is_auspicious_day(&self) -> bool;
}
```

---

## 5. `no_std` Strategy

| Crate / Module | Policy | Justification |
|---|---|---|
| `calendar-core` | `no_std + alloc` | All types are plain data; no OS interaction |
| [`balinese-calendar`](https://github.com/SHA888/balinese-calendar) | `no_std + alloc` | Pure arithmetic (crate: https://crates.io/crates/balinese-calendar) |
| `nusantara-calendar` (crate root) | `no_std + alloc` | Inherits from feature-gated modules |
| `balinese` module | `no_std + alloc` | Wraps no_std external crate |
| `jawa` module | `no_std + alloc` | Pure modular arithmetic on `const` tables |
| `hijriyah` module | `no_std + alloc` | Tabular algorithm from D-R Ch. 6; no float |
| `chinese_nusantara` module | `std` required | `nongli` → `chrono` → `std` |
| `batak` module (tabular) | `no_std + alloc` | Tabular Porhalaan; lookup tables as `const` |
| `batak` module (astronomical) | `std` required | `libm` or `std::f64` for heliacal angle calc |
| `sunda`, `tengger` modules | `no_std + alloc` | Saka arithmetic |
| `bugis`, `sasak`, `dayak`, `toraja`, `minangkabau` modules | `no_std + alloc` | Cycle arithmetic; stubs for unverified parts |
| `dewasa_engine` module | `std` required | `HashMap<Activity, CrossCalendarVerdict>` |

**Implementation pattern for modules that need both:**

```toml
# nusantara-calendar Cargo.toml
[features]
default = ["std"]
std = []
astronomical = ["std", "libm"]
```

```rust
// lib.rs
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
```

**`alloc` usage is permitted everywhere.** `String` (for error messages), `Vec` (for
dynamic verdict lists in `dewasa_engine`), and `Box<dyn Error>` are all fine. The
constraint is *no `std`*, not *no heap*.

---

## 6. Feature Flag System

`nusantara-calendar` uses feature flags to enable calendar modules:

```toml
[features]
default = ["std"]
std     = []
serde   = ["dep:serde", "calendar-core/serde"]
wasm    = ["calendar-core/wasm-bindgen", "wasm-bindgen"]
astronomical = ["std", "dep:libm"]

# Calendar modules
balinese         = ["balinese-calendar"]
jawa             = ["std"]
hijriyah         = ["std"]
batak            = ["std"]
sunda            = ["std"]
tengger          = ["std"]
bugis            = ["std"]
sasak            = ["std"]
dayak            = ["std"]
toraja           = ["std"]
minangkabau      = ["std"]
chinese-nusantara = ["nongli"]
dewasa-engine    = ["std"]

all-calendars = ["balinese", "jawa", "hijriyah", "batak", "sunda", "tengger",
                  "bugis", "sasak", "dayak", "toraja", "minangkabau", "chinese-nusantara"]
all           = ["all-calendars", "dewasa-engine"]
```

**Flag semantics:**

- `std`: unlocks `std::error::Error` impl on `CalendarError` and `std::fmt::Display`
  improvements. Required by most calendar modules.
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
    ($msg:expr) => {
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

Three modules have calendar cycles that depend on heliacal astronomical events that
cannot be computed by pure tabular arithmetic:

| Module | Observation event | Observer latitude |
|---|---|---|
| `batak` | Orion (Waluku) / Scorpius first rise | Lake Toba, ~2.6°N |
| `sasak` | Pleiades (Bintang Rowot) first rise | Lombok, ~8.6°S |
| `bugis`, `minangkabau` | Pleiades (Bintang Kartika) visibility | Sulawesi ~4°S / Sumatra ~0° |

**Dual-mode API pattern (mandatory for all observation-dependent modules):**

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

All `no_std + alloc` modules must compile cleanly to `wasm32-unknown-unknown`:

```sh
# Verified in CI
cargo build --target wasm32-unknown-unknown -p calendar-core --no-default-features
cargo build --target wasm32-unknown-unknown -p nusantara-calendar \
  --no-default-features --features balinese,jawa,hijriyah
```

`chinese-nusantara` and `dewasa-engine` do **not** have a WASM build target. This is
expected and is documented in `Cargo.toml` with:

```toml
[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]  # WASM not supported when these features are enabled
```

**WASM `wasm_bindgen` exports** (when `wasm` feature is enabled) expose a flat,
date-string-based API suitable for JavaScript consumers of `dedauh.id`:

```rust
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn balinese_from_gregorian(y: i32, m: u8, d: u8) -> Result<JsValue, JsValue> {
    BalineseDate::from_gregorian(y, m, d)
        .map(|d| serde_wasm_bindgen::to_value(&d).unwrap())
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
```

---

## 11. `dewasa-engine` Aggregation Model

`dewasa_engine` is the only module that imports more than one calendar module. Its
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
(i.e., one `Auspicious`/`VeryAuspicious` and another `Inauspicious`/`VeryInauspicious`).

**Consensus rule (default):** `overall` = the most cautious level across all calendars
that returned a non-`Unknown` result.

---

## 12. Dependency Decisions & Licenses

| Dependency | Version | License | Used by | Decision |
|---|---|---|---|---|
| `thiserror` | 2.x | MIT OR Apache-2.0 | `calendar-core` | Standard error derive |
| `balinese-calendar` | 0.2.x | MIT | `nusantara-calendar` (`balinese` feature) | Official Balinese implementation |
| `nongli` | 0.4.x | MIT | `nusantara-calendar` (`chinese-nusantara` feature) | Only Rust Chinese lunisolar crate with compatible API |
| `chrono` | 0.4.x | MIT OR Apache-2.0 | transitive via `nongli` | `std` feature only |
| `serde` | 1.x | MIT OR Apache-2.0 | all (optional `serde` feature) | De-facto serialization standard |
| `wasm-bindgen` | 0.2.x | MIT OR Apache-2.0 | all (optional `wasm` feature) | Required for WASM JS interop |
| `libm` | 0.2.x | MIT | `batak`, `sasak`, `bugis`, `minangkabau` (optional `astronomical`) | `no_std`-compatible float math |

**Explicitly excluded:**

- `misykat` (GPL-3.0-only) — copyleft; would force GPL-3.0-only terms on all dependents. Hijri arithmetic
  reimplemented independently from Dershowitz-Reingold Ch. 6 and Meeus Ch. 9 to keep the workspace permissive.
- `tanggalan` (bect/tanggalan) — does not exist on crates.io as of March 2026.
- Any crate providing astrology, prayer times, or timezone conversion — out of scope.

---

## 13. Error Taxonomy

```rust
pub enum CalendarError {
    OutOfRange(String),         // Date outside the module's supported year range
    InvalidParameters(String),  // Malformed input (wrong month number, day > 31, etc.)
    NotImplemented(String),     // Algorithm known but not yet coded; stub!() target
    ArithmeticError(String),    // Internal calculation overflow or undefined result
}
```

**`OutOfRange` usage:** Each module documents its supported year range. E.g., `jawa`
supports AJ 1555–2474 (Gregorian 1633–2169, spanning the current and next kurup).
Dates outside this range return `OutOfRange`, not a silently wrong result.

**`InvalidParameters` usage:** For inputs that are structurally invalid — month 13,
day 0, negative day. Distinct from `OutOfRange` (in-range but structurally bad input).

**`ArithmeticError` usage:** Reserved for internal calculation failures — integer
overflow on an intermediate value, undefined modulo, etc. Should not be reachable on
valid input; indicates a bug in the implementation.

**`NotImplemented` usage:** Returned by `stub!()`. Indicates the algorithm exists in
principle but has not been coded yet. Consumers should handle this case gracefully.
Stub-to-implementation upgrades are not breaking changes.

---

## 14. Versioning & Stability Guarantees

Follows [Semantic Versioning 2.0.0](https://semver.org/).

| Version series | Scope | API stability |
|---|---|---|
| 0.x | `calendar-core` + `nusantara-calendar` pre-1.0 | Breaking changes allowed between minors |
| 1.0.0 | All modules (post dewasa-engine stabilization) | `CalendarDate`, `CalendarMetadata`, `CalendarError` are stable |
| 1.x | All modules | Additive only: new calendar modules, new `Activity` variants (non-exhaustive), stub → impl upgrades |

**`#[non_exhaustive]`** is applied to `Activity` and `AuspiciousnessLevel`. Consumers
must handle `_` arms. This allows adding new activities without a semver break.

**Stub-to-implementation upgrades** (e.g., `stub!()` → real algorithm) are **not**
considered breaking changes, even though they change return values. The previous return
was `Err(NotImplemented)`. Any consumer that was pattern-matching on that and treating
it as "OK, no data" will now receive `Ok(data)` instead — this is strictly additive.
Document these upgrades in `CHANGELOG.md` under `### Added`.
