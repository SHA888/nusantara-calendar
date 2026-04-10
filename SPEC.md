# `nusantara-calendar` Workspace Specification — v3

> **Changelog from v2**
> - v3: Workspace structure corrected to actual 2-crate layout (calendar-core + nusantara-calendar with feature-gated modules). balinese-calendar remains an external standalone crate. CalendarDate, CalendarMetadata, HasAuspiciousness, Activity, AuspiciousnessLevel, and CalendarError definitions updated to match published calendar-core v0.1.0 implementation.
>
> **Changelog from v1**
> - Fix 1: Sultan Agung epoch corrected to 1633-07-08, JDN 2317690
> - Fix 2: `misykat` GPL-3.0 licensing constraint documented with resolution paths
> - Fix 3: `bect/tanggalan` (non-existent) removed; `jawa` defaults to direct implementation
> - Fix 4: `no_std` scope tightened — core traits `no_std + alloc`, wrapper crates `std` via feature flag
> - Fix 5: "Sancaya" corrected — it is a supra-windu group name, not a windu year name; current windu year is Wawu (AJ 1959)

---

## Vision

Build a Rust workspace — `nusantara-calendar` — covering every traditional calendar system
with documented algorithmic basis across the Indonesian archipelago, plus the three supra-ethnic
calendars (Hijriyah, Javanese, Chinese) present on standard Indonesian almanacs. This is the
only such library in any language with Rust as its implementation target.

Scope is archipelago-wide: from Sabang (Aceh) to Merauke (Papua), covering all ethnic groups
with documented timekeeping systems. Where a calendar system has no published algorithm, the
crate must document what IS known and expose a `stub!()` placeholder with source attribution,
rather than silently omitting or guessing.

The workspace will eventually power `dedauh.id` — a multi-calendar SaaS platform — and must
be API-stable, WASM-compilable, and publishable to crates.io.

---

## Workspace Structure

The workspace contains **two crates**. All calendar systems are feature-gated modules within
`nusantara-calendar`, not separate crates. `balinese-calendar` is an external standalone crate
in its own repository.

```
nusantara-calendar/              ← workspace root
├── Cargo.toml                   ← workspace root; lists calendar-core + nusantara-calendar
├── crates/
│   ├── calendar-core/           ← published v0.1.0: shared traits, JDN pivot, error types
│   │   └── src/lib.rs
│   └── nusantara-calendar/      ← not yet published; umbrella crate with feature-gated modules
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── balinese/        ← wraps external balinese-calendar v0.2.0
│           ├── jawa/            ← stub
│           ├── hijriyah/        ← stub
│           ├── batak/           ← stub
│           ├── sunda/           ← stub
│           ├── tengger/         ← stub
│           ├── bugis/           ← stub
│           ├── sasak/           ← stub
│           ├── dayak/           ← stub
│           ├── toraja/          ← stub
│           ├── minangkabau/     ← stub
│           ├── chinese_nusantara/ ← stub (will wrap nongli)
│           └── dewasa_engine/   ← stub (std-only)
```

**External dependency (separate repo):**
- `balinese-calendar` v0.2.0 — https://github.com/SHA888/balinese-calendar
  — https://crates.io/crates/balinese-calendar — MIT license — no_std + alloc

---

## Crate Specifications

### `calendar-core` — v0.1.0 (published)

Universal pivot and shared contracts. No calendar logic lives here.

#### `no_std` policy

`calendar-core` is `no_std + alloc`. All types are plain data.

Modules within `nusantara-calendar` that require `std` (chinese_nusantara, dewasa_engine)
must be gated behind the `std` feature and documented explicitly.

#### Types

```rust
pub type JDN = i64;             // Julian Day Number
pub type CycleYear = u32;       // Calendar cycle years
pub type SubYearPosition = u8;  // Sub-year positions (month, day, weekday)
```

#### JDN Functions

```rust
/// Meeus, Astronomical Algorithms, Ch. 7
pub fn gregorian_to_jdn(year: i32, month: u8, day: u8) -> JDN { ... }

/// Fliegel & van Flandern (1968), USNO algorithm
pub fn jdn_to_gregorian(jdn: JDN) -> (i32, u8, u8) { ... }
```

Verification anchors (both tested in `calendar-core`):
- `gregorian_to_jdn(1582, 10, 15) == 2299161` — Gregorian reform date
- `gregorian_to_jdn(1633, 7, 8) == 2317690` — Sultan Agung epoch

#### `CalendarDate` Trait

```rust
pub trait CalendarDate: Clone + PartialEq + Eq + core::fmt::Debug {
    fn from_jdn(jdn: JDN) -> Result<Self, CalendarError>
    where Self: Sized;

    fn to_jdn(&self) -> JDN;

    fn calendar_name() -> &'static str;

    fn validate_range(&self) -> Result<(), CalendarError>;

    // Default impls via JDN — override only for non-standard Gregorian ↔ JDN mappings
    fn from_gregorian(year: i32, month: u8, day: u8) -> Result<Self, CalendarError>
    where Self: Sized;

    fn to_gregorian(&self) -> (i32, u8, u8);
}
```

#### `CalendarMetadata` Trait

```rust
pub trait CalendarMetadata {
    fn epoch() -> JDN;
    fn cycle_length() -> Option<CycleYear> { None }
    fn description() -> &'static str;
    fn cultural_origin() -> &'static str;
}
```

#### `HasAuspiciousness` Trait

```rust
pub trait HasAuspiciousness {
    type Activity;
    type AuspiciousnessLevel;

    fn auspiciousness_for(&self, activity: &Self::Activity) -> Self::AuspiciousnessLevel;
    fn is_auspicious_day(&self) -> bool;
}
```

#### `Activity` Enum

```rust
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Activity {
    Marriage,
    Building,
    Travel,
    Business,
    Agriculture,
    ReligiousCeremony,
    Naming,
    MovingHouse,
    Education,
    Medical,
    Custom(String),
}
```

#### `AuspiciousnessLevel` Enum

```rust
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AuspiciousnessLevel {
    VeryAuspicious,
    Auspicious,
    Neutral,
    Inauspicious,
    VeryInauspicious,
}
```

#### `CalendarError`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarError {
    OutOfRange(String),         // Date outside the module's supported year range
    InvalidParameters(String),  // Malformed input
    NotImplemented(String),     // stub!() target — algorithm known but not coded
    ArithmeticError(String),    // Internal calculation failure
}
```

#### `stub!()` Macro

```rust
#[macro_export]
macro_rules! stub {
    ($msg:expr) => {
        return Err($crate::CalendarError::NotImplemented($msg.to_string()))
    };
}
```

---

### `nusantara-calendar` — v0.1.0 (not yet published)

Umbrella crate. Re-exports `calendar-core` traits and provides feature-gated calendar modules.

```toml
[features]
default = ["std"]
std     = ["thiserror", "calendar-core/std"]
serde   = ["dep:serde", "calendar-core/serde"]
wasm    = ["calendar-core/wasm-bindgen", "wasm-bindgen"]

balinese          = ["balinese-calendar"]
jawa              = ["std"]
hijriyah          = ["std"]
batak             = ["std"]
sunda             = ["std"]
tengger           = ["std"]
bugis             = ["std"]
sasak             = ["std"]
dayak             = ["std"]
toraja            = ["std"]
minangkabau       = ["std"]
chinese-nusantara = ["nongli"]
dewasa-engine     = ["std"]

all-calendars = [all individual calendar features]
all           = ["all-calendars", "dewasa-engine"]
```

---

### `balinese` module

Wrapper around `balinese-calendar` v0.2.0 (external crate). Implements `CalendarDate`,
`CalendarMetadata`, `HasAuspiciousness` from `calendar-core` on a `BalineseDate` newtype
that wraps `balinese_calendar::BalineseDate`.

**Status:** Implemented. Tested (9 unit tests + 2 doc tests).

---

### `jawa` module

#### Epoch

```rust
/// Epoch of the Sultan Agung Javanese calendar reform.
/// Gregorian 1633-07-08 = 1 Sura 1555 AJ = 1 Muharram 1043 AH.
///
/// Sources: Beauducel & Karjanto (2020), arXiv:2012.10064;
///          Wikipedia "Javanese calendar" (verified March 2026).
pub const SULTAN_AGUNG_EPOCH_JDN: i64 = 2317690;
```

#### Dependency policy

No `tanggalan` crate exists in Rust on crates.io (as of March 2026). The only
`tanggalan` repository found (`mohamadrido/tanggalan`) is JavaScript/HTML. The `jawa`
module must be implemented independently.

Algorithmic sources:
- Beauducel & Karjanto (2020), arXiv:2012.10064 (Wetonan congruence formula)
- Dershowitz & Reingold, *Calendrical Calculations* (4th ed.), Ch. 10 (Pawukon 210-day)
- H. Danudji, *Penanggalan Jawa 120 Tahun Kurup Asapon*, Dahara Prize 2006 (Kurup table)

Required cycles:
- **Wetonan**: Saptawara (7) × Pasaran (5) = 35-day cycle; neptu values for both
- **Pawukon**: 30-wuku × 7-day = 210-day cycle (Dershowitz-Reingold Ch. 10)
- **Windu**: 8-year cycle. Names: Alip, Ehe, Jimawal, Je, Dal, Be, Wawu, Jimakir.
  Leap years: Jimawal (355 days), Dal (355 days), Jimakir (355 days); others 354 days.
- **Kurup**: 120-year cycle (15 windus). Current: Alip Selasa Pon,
  started 1936-03-24, ends 2052-08-25 (Danudji 2006).
- **Wuku**: same 30-wuku structure as Balinese; Javanese-language names
- **Pranata Masa**: Kasa, Karo, Katelu, Kapat, Kalima, Kanem, Kapitu, Kawolu,
  Kasongo, Kasepuluh, Desta, Sada (12 solar seasons)

#### Windu year naming

**"Sancaya" is not a Windu year name.** It is a supra-windu group name within the
120-year Kurup hierarchy. The 8-name Windu year sequence is Alip through Jimakir.
As of AJ ~1959 (March 2026), the current Windu year is **Wawu**.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinduYear {
    Alip = 1,
    Ehe = 2,
    Jimawal = 3,  // leap (355 days)
    Je = 4,
    Dal = 5,      // leap (355 days)
    Be = 6,
    Wawu = 7,
    Jimakir = 8,  // leap (355 days)
}

impl WinduYear {
    pub fn is_leap(&self) -> bool {
        matches!(self, Self::Jimawal | Self::Dal | Self::Jimakir)
    }
    pub fn from_aj(aj: u32) -> Self {
        match (aj - 1) % 8 {
            0 => Self::Alip, 1 => Self::Ehe, 2 => Self::Jimawal,
            3 => Self::Je,   4 => Self::Dal, 5 => Self::Be,
            6 => Self::Wawu, _ => Self::Jimakir,
        }
    }
}
```

---

### `hijriyah` module

**GPL-3.0 licensing blocker:** `misykat` v4.1.2 is GPL-3.0-only. Using it would force
GPL-3.0 on `hijriyah`, `dewasa_engine`, and all downstream consumers, incompatible with
the workspace's `MIT OR Apache-2.0` license.

**Resolution (Option A — selected):** Reimplement Hijri arithmetic independently from
Dershowitz & Reingold Ch. 6 and Meeus Ch. 9. Do not read `misykat` source. Record
the exclusion rationale in `hijriyah/DECISION.md`.

Design requirements:
- `no_std + alloc` when `std` feature is disabled
- Public API: `HijriDay`, `HijriMonth` (Arabic + Indonesian names), `Pasaran`
- Tabular vs government date (`indonesian_government_date()` → `stub!()`)
- Supported range: 1–1600 AH minimum
- Anchor: `hijri_to_jdn(1, 1, 1) == 1948439` (Thursday epoch, D-R Eq. 6.2–6.3)

---

### `chinese_nusantara` module

Thin wrapper over `nongli` v0.4.1 (`github.com/supertsy5/nongli`, MIT license).
**`std`-only** — `nongli` depends on `chrono` which requires `std`.

Peranakan-specific additions:
- `Shio` enum (12-year zodiac, Indonesian names)
- `cap_go_meh_jdn(chinese_year)` — 15th day, 1st month
- `imlek_jdn(chinese_year)` — 1st day, 1st month
- Weton Tionghoa (Pasaran intersection, same `(jdn + 2) % 5` formula as `hijriyah`)
- Document Singkawang vs Solo/Semarang Cap Go Meh variants

License audit note: `nongli` v0.4.1 is MIT — compatible. Confirm via `cargo deny check`
before publishing.

---

### `batak` module

Porhalaan: lunar-star system. 12-month years and 13-month leap years, keyed to
Orion (Bintang Waluku) and Scorpius visibility.

Dual-mode API (mandatory):
```rust
pub fn from_jdn_tabular(jdn: i64) -> Result<Self, CalendarError>;  // no_std
#[cfg(feature = "astronomical")]
pub fn from_jdn_astronomical(jdn: i64, lat: f64, lon: f64) -> Result<Self, CalendarError>;
```

Toba month names (12 + intercalary):
Sipaha Sada, Sipaha Dua, Sipaha Tolu, Sipaha Opat, Sipaha Lima, Sipaha Onom,
Sipaha Pitu, Sipaha Ualu, Sipaha Sia, Sipaha Sampulu, Hapungan, Hurung (intercalary: Ihuthon).

Primary source: "A Lunar-Star Calendar: Inquiry to the Traditional Batak Calendar",
preprints.org/manuscript/202404.0235 (2024).

---

### `sunda` module

- Kala Sunda (pre-Islamic Saka-derived)
- Pranatamangsa Sunda (12 agricultural seasons; different month names and solar epoch from Javanese)
- Naga Tahun cycle (year-direction taboo)
- Sunda Wiwitan ceremonial calendar → `stub!()` (Baduy oral tradition; cite Garna 1993)

---

### `bugis` module

- 12-month Islamic-influenced lunar calendar; pre-Islamic month names retained
- `tudang_sipulung_jdn(year)` — Pleiades first rise, ~3°S; tabular fallback
- Siri' timing via lunar phase
- Source: Pelras, *The Bugis* (1996)

---

### `sasak` module

- Rowot calendar: new year keyed to Pleiades (Bintang Rowot) first rise, Lombok ~8.6°S
- `bau_nyale_jdn(year)` — 10th month, specific lunar day
- Source: Taufiq et al. — Lombok traditional astronomy

---

### `dayak` module

- Kaharingan 12-month agricultural cycle; Pleiades-keyed and rice-cultivation phases
- Tewah ritual timing: lunar phase + agricultural cycle
- `const` Ngaju month names; stub Kayan/Kenyah/Murut variants
- Source: Schärer, *Ngaju Religion* (1963)

---

### `toraja` module

- 12 lunar months; Toraja-language names
- Rambu Solo' (death ritual) and Rambu Tuka' (life ritual) seasons
- `manene_jdn(year)` — August cycle, post-harvest lunar phase
- Source: Nooy-Palm, *The Sa'dan Toraja* Vol. 1 (1979)

---

### `tengger` module

- Closest living relative of pre-Islamic Javanese Hindu calendar; Saka-based
- `kasada_jdn(tengger_year)` — 14th day, Kasada month (annual Bromo pilgrimage)
- **Unan-unan**: 5-year purification cycle — distinct from Javanese Windu; do not conflate
- Source: Hefner, *Hindu Javanese: Tengger Tradition and Islam* (1985)

---

### `minangkabau` module

- Primarily Islamic (Hijriyah) with agricultural overlay
- `turun_ka_sawah_jdn(year)` — Pleiades visibility + lunar month, tabular fallback ~0°
- `hari_raya_adat_jdn(year)` → `stub!()` with citation
- Source: Kato, *Matriliny and Migration* (1982)

---

### `dewasa_engine` module

**`std`-only** — uses `HashMap` for verdict aggregation. Explicitly exempt from `no_std`.

```rust
pub struct NusantaraDay {
    pub gregorian: (i32, u8, u8),
    pub jdn: i64,
    pub balinese: Option<balinese::BalineseDate>,
    pub jawa: Option<jawa::JavaneseDay>,
    pub hijriyah: Option<hijriyah::HijriDay>,
    pub chinese: Option<chinese_nusantara::ChineseNusantaraDay>,
    // feature-gated ethnic modules...
    pub cross_auspiciousness: HashMap<Activity, CrossCalendarVerdict>,
}

pub struct CrossCalendarVerdict {
    pub overall: AuspiciousnessLevel,
    pub by_calendar: HashMap<&'static str, AuspiciousnessLevel>,
    pub conflicts: Vec<CalendarConflict>,
    pub consensus_notes: Vec<&'static str>,
}
```

---

## Technical Constraints

| Module | `no_std + alloc` | `std` required | Reason |
|---|---|---|---|
| `calendar-core` | ✅ | — | trait definitions, JDN math |
| [`balinese-calendar`](https://github.com/SHA888/balinese-calendar) | ✅ | — | pure arithmetic |
| `balinese` | ✅ | — | wraps no_std external crate |
| `jawa` | ✅ | — | pure arithmetic |
| `hijriyah` | ✅ | — | tabular arithmetic, Option A (independent) |
| `chinese_nusantara` | ❌ | ✅ | `nongli` → `chrono` → `std` |
| `batak` (tabular) | ✅ | — | const lookup tables |
| `batak` (astronomical) | ❌ | ✅ | float math via `libm` |
| `sunda`, `tengger`, `bugis`, `sasak`, `dayak`, `toraja`, `minangkabau` | ✅ | — | pure arithmetic / stubs |
| `dewasa_engine` | ❌ | ✅ | `HashMap` aggregation |

Additional constraints applying to all modules:
- Rust edition 2021, MSRV 1.80
- All static lookup tables as `const` — no runtime heap allocation for data
- WASM32 target must compile for all `no_std` modules
- Feature flags: `serde`, `wasm`, `astronomical` (for observation-dependent modules)
- Every calendar module implements `CalendarMetadata` from `calendar-core`
- Where algorithm is unknown/unconfirmed, use `stub!()` — never silently fabricate data

---

## Build Order (strict — each module depends on previous)

1. `calendar-core` (published) — traits, JDN math, error types, `stub!()` macro
2. `balinese` module (wraps external `balinese-calendar` v0.2.0)
3. `jawa` module + `hijriyah` module + `chinese_nusantara` module
4. `batak` module (most academically documented after Bali/Jawa)
5. `sunda` module + `tengger` module (algorithmically close to Jawa/Bali)
6. `bugis` module + `sasak` module + `dayak` module (observation-dependent, partial stubs)
7. `toraja` module + `minangkabau` module (Islamic overlay + partial stubs)
8. `dewasa_engine` module (depends on all above; `nusantara-calendar` v1.0 gate)

---

## Documentation Standard

Every public struct/enum/fn must have:
```rust
/// **[Ethnonym]** — *[Native-language name]* ([transliteration])
///
/// [One-sentence description in English]
///
/// # Calendar System
/// [Calendar name, ethnic group, region]
///
/// # Sources
/// - [Citable reference 1 with author, title, year, ISBN/DOI/URL]
/// - [Citable reference 2]
```

---

## What This Is NOT

- Not an astrology app
- Not a prayer-time calculator (use `misykat` or `praytime-rs` for that)
- Not a Gregorian calendar utility
- Not a timezone converter

This is purely a **temporal computation library** for Indonesian traditional calendar systems,
with algorithmic correctness and source attribution as the primary quality metrics.
