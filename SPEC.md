# `nusantara-calendar` Workspace Specification — v2

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
be API-stable, WASM-compilable, and publishable to crates.io as independent crates.

---

## Workspace Structure

```
nusantara-calendar/
├── Cargo.toml                    # workspace root
├── crates/
│   ├── calendar-core/            # shared traits, JDN pivot, error types
│   ├── balinese-calendar/        # existing crate (git submodule or local path)
│   ├── jawa/                     # Javanese Pawukon + Wetonan + Windu + Pranata Masa
│   ├── hijriyah/                 # Islamic lunar (thin wrapper + Indonesian extensions)
│   ├── chinese-nusantara/        # Chinese lunisolar (Peranakan context)
│   ├── batak/                    # Batak Porhalaan (Toba, Karo, Simalungun variants)
│   ├── sunda/                    # Sundanese Kala Sunda / Pranatamangsa Sunda
│   ├── bugis/                    # Bugis-Makassar lunar calendar
│   ├── sasak/                    # Sasak Lombok traditional calendar
│   ├── dayak/                    # Dayak Kaharingan agricultural calendar
│   ├── toraja/                   # Toraja ritual calendar
│   ├── tengger/                  # Tengger (Bromo) Hindu calendar variant
│   ├── minangkabau/              # Minangkabau agricultural/ceremonial cycles
│   └── dewasa-engine/            # cross-calendar auspiciousness correlator
```

---

## Crate Specifications

### `calendar-core`

Universal pivot and shared contracts.

#### `no_std` policy

`calendar-core` and all computation kernels (pure arithmetic, lookup tables, cycle
arithmetic) **must** be `no_std + alloc` compatible. The `CalendarDate` trait and `CalendarError`
type live here and are therefore `no_std`.

Wrapper crates (`hijriyah`, `chinese-nusantara`) depend on external crates that require `std`.
They **must** declare `std` as a non-optional dependency and be clearly marked in their
`Cargo.toml`:

```toml
# In hijriyah/Cargo.toml and chinese-nusantara/Cargo.toml
[features]
default = ["std"]
std = []  # non-optional; documented as such

[dependencies]
# misykat / nongli require std; this crate is std-only
```

`dewasa-engine` is also `std`-only (uses `HashMap` for verdict aggregation) and must
document this explicitly. It is **not** subject to the `no_std` requirement.

```rust
/// Julian Day Number is the universal interop format.
/// All calendar implementations convert through JDN.
pub trait CalendarDate: Sized + Clone + PartialEq {
    fn from_jdn(jdn: i64) -> Result<Self, CalendarError>;
    fn to_jdn(&self) -> i64;

    // Convenience: default impl via JDN
    fn from_gregorian(y: i32, m: u8, d: u8) -> Result<Self, CalendarError> {
        let jdn = gregorian_to_jdn(y, m, d);
        Self::from_jdn(jdn)
    }
    fn to_gregorian(&self) -> (i32, u8, u8) {
        jdn_to_gregorian(self.to_jdn())
    }
}

pub trait HasAuspiciousness {
    fn auspiciousness(&self, activity: Activity) -> AuspiciousnessLevel;
}

pub trait CalendarMetadata {
    fn calendar_name() -> &'static str;          // e.g. "Kalender Bali"
    fn ethnic_group() -> &'static str;           // e.g. "Bali"
    fn region() -> &'static str;                 // e.g. "Bali, Indonesia"
    fn epoch_jdn() -> i64;                       // calendar epoch as JDN
    fn reference_sources() -> &'static [&'static str]; // citable sources
}

#[non_exhaustive]
pub enum Activity {
    Marriage, BuildingStart, FarmingStart, Travel,
    MedicalProcedure, Cremation, BusinessLaunch,
    HarvestStart, FishingStart, SeaVoyage,
    ReligiousCeremony, AncestorRitual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuspiciousnessLevel {
    HighlyAuspicious, Auspicious, Neutral, Inauspicious, Forbidden,
    Unknown,    // calendar exists but rule not yet digitized
}

#[derive(Debug, thiserror::Error)]
pub enum CalendarError {
    #[error("date out of supported range: {0}")]
    OutOfRange(String),
    #[error("algorithm not yet implemented: {0}")]
    NotImplemented(String),
    #[error("source data ambiguous: {0}")]
    Ambiguous(String),
}

/// Convenience macro for unimplemented stubs.
/// Expands to an early return of `CalendarError::NotImplemented`.
#[macro_export]
macro_rules! stub {
    ($msg:literal) => {
        return Err($crate::CalendarError::NotImplemented($msg.to_string()))
    };
}
```

Provide: `gregorian_to_jdn(y, m, d) -> i64` and `jdn_to_gregorian(jdn) -> (i32, u8, u8)`
using the standard proleptic Gregorian algorithm (Meeus, *Astronomical Algorithms* Ch. 7).

---

### `jawa` (Javanese Calendar)

#### Epoch

> **[Fix 1]** The Sultan Agung calendar reform commenced on **8 July 1633 CE** (Gregorian),
> corresponding to 1 Sura 1555 AJ, 1 Muharram 1043 AH.
>
> Verified JDN (Meeus proleptic Gregorian): **2317690**.
>
> Sources: Beauducel & Karjanto (2020), arXiv:2012.10064; academic PDF "Sultan Agung's Thought
> of Javanese Islamic Calendar" (Academia.edu, 2021); Wikipedia "Javanese calendar" (verified
> March 2026 against physical almanac cross-reference).

```rust
/// Epoch of the Sultan Agung Javanese calendar reform.
/// Gregorian 1633-07-08 = 1 Sura 1555 AJ = 1 Muharram 1043 AH.
///
/// Computed via Meeus proleptic Gregorian algorithm (Astronomical Algorithms, Ch. 7).
/// Cross-verified against Beauducel & Karjanto (2020), arXiv:2012.10064.
pub const SULTAN_AGUNG_EPOCH_JDN: i64 = 2317690;
```

#### Dependency policy

> **[Fix 3]** The crate `tanggalan` at `github.com/bect/tanggalan` described in v1 of this
> spec does not exist on crates.io or GitHub as of March 2026. The only `tanggalan` repository
> found (`mohamadrido/tanggalan`) is a JavaScript/HTML project with no Rust code.
>
> **The `jawa` crate must be implemented independently.** Algorithmic sources:
> - Dershowitz & Reingold, *Calendrical Calculations* (4th ed.), Ch. 10 "The Balinese Pawukon
>   Calendar" (for 210-day Pawukon shared with Balinese)
> - Beauducel & Karjanto (2020), arXiv:2012.10064 (Wetonan congruence formula; verified
>   against physical implementation at `github.com/beaudu/weton`)
> - Wikipedia "Javanese calendar" (cites H. Danudji, *Penanggalan Jawa 120 Tahun Kurup
>   Asapon*, Dahara Prize 2006, ISBN 979-501-454-4) for Kurup table

Required cycles:
- **Wetonan**: Saptawara (7) × Pasaran (5) = 35-day cycle; neptu values for both
- **Pawukon**: 30-wuku × 7-day = 210-day cycle (shared with Balinese; reference
  Dershowitz-Reingold Ch. 10 "The Balinese Pawukon Calendar")
- **Windu**: 8-year cycle. Names in sequence: Alip, Ehe, Jimawal, Je, Dal, Be, Wawu, Jimakir.
  Leap years within windu: Jimawal (355 days), Dal (355 days), Jimakir (355 days);
  all others 354 days.
- **Kurup**: 120-year cycle (15 windus). Current kurup = Alip Selasa Pon
  (started 1936-03-24 CE = 1 Muharram 1355 AH, ends 2052-08-25 CE;
  source: Wikipedia "Javanese calendar", citing H. Danudji 2006).
- **Wuku**: same 30-wuku structure as Balinese but Javanese-language names
- **Pranata Masa**: 12 agricultural seasons (solar-based, 12 × ~30 days).
  Names: Kasa, Karo, Katelu, Kapat, Kalima, Kanem, Kapitu, Kawolu,
  Kasongo, Kasepuluh, Desta, Sada.
- **Dina Mulya**: noble days for Kejawen practitioners

#### Windu year naming — clarification

> **[Fix 5]** v1 of this spec incorrectly stated "current windu = Sancaya".
>
> **"Sancaya" (also spelled Sangara, Sêngara) is a supra-windu group name, not a windu year
> name.** Within the 120-year Kurup, windus are grouped into named sets at a higher hierarchy
> level (e.g., "Kuntara", "Sengara", "Langkir", "Adi" in some traditional sources). These
> group names appear in full almanac date notation alongside the individual windu year name.
>
> The windu **year** name (position within the 8-year cycle) is one of the eight names listed
> above. As of March 2026 (AJ ≈ 1959), the current windu year name is **Wawu** (7th position).
>
> Implementations must:
> 1. Expose the 8-name windu year cycle (`WinduYear` enum: `Alip` through `Jimakir`)
> 2. Not conflate it with supra-windu group names, which are not algorithmically standardized
>    across sources and should be left to `stub!()` pending a citable primary source

```rust
/// Position within the 8-year Windu cycle.
/// Leap year (355 days) occurs in Jimawal, Dal, and Jimakir.
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

    /// Compute from Anno Javanico year number.
    pub fn from_aj(aj: u32) -> Self {
        match (aj - 1) % 8 {
            0 => Self::Alip,
            1 => Self::Ehe,
            2 => Self::Jimawal,
            3 => Self::Je,
            4 => Self::Dal,
            5 => Self::Be,
            6 => Self::Wawu,
            _ => Self::Jimakir,
        }
    }
}
```

---

### `hijriyah`

> **[Fix 2 — LICENSING BLOCKER]** — Full execution plan: `~/.windsurf/plans/hijriyah-implementation-a99a87.md`.

**Problem.** `misykat` v4.1.2 (GPL-3.0-only) cannot be a dependency. Pulling it in would force
GPL-3.0-only on `hijriyah`, `dewasa-engine`, and downstream consumers, contradicting the workspace's
`MIT OR Apache-2.0` license.

**Decision (Option A).** Reimplement Hijri arithmetic independently from primary sources
(Dershowitz & Reingold, *Calendrical Calculations* 4th ed. Ch. 6; Meeus, *Astronomical Algorithms* 2nd ed. Ch. 9).
Record this in `hijriyah/DECISION.md` and cite both sources in `SOURCES.md`. Do **not** read misykat code.

#### Design requirements

- Crate metadata: `name = "hijriyah"`, version `0.2.0`, `edition = 2021`, `rust-version = 1.75`, `license = "MIT OR Apache-2.0"`.
- Feature flags: `default = []`, `std`, `serde`, `wasm`. Must compile `no_std + alloc` when `std` is disabled.
- Public API exposes `HijriDay`, `HijriMonth`, `Pasaran`, tabular vs government date distinction, Indonesian holidays, and haul computation.
- Supported range: at least 1–1600 AH. Document behavior outside range (`CalendarError::OutOfRange`).
- Documentation style: every public item gets the Islamic context docstring (Arabic + Indonesian name, epoch, sources).

#### Implementation scope

1. **Skeleton & gating**: `crates/hijriyah/` with `Cargo.toml`, `DECISION.md`, `SOURCES.md`, `src/lib.rs`, `arithmetic.rs`, `types.rs`, `holidays.rs`, `metadata.rs`, `tests/anchors.rs`.
   - `#![cfg_attr(not(feature = "std"), no_std)]`, `extern crate alloc`, `pub use` key types.
2. **Core arithmetic**: Implement `hijri_to_jdn` / `jdn_to_hijri` using D-R Eq. 6.2–6.3 (Thursday epoch, JDN 1948439).
   - Leap-year cycle: years {2,5,7,10,13,16,18,21,24,26,29} per 30-year block. Expose `HijriDay::is_leap_year`.
3. **Types**: Build `HijriDay` struct with year/month/day, `HijriMonth` enum (Arabic + Indonesian names), `Pasaran` enum (Kliwon–Wage) using `(jdn + 2) % 5` formula.
4. **Indonesian extensions**: Provide `maulid_jdn`, `isra_miraj_jdn`, `idul_fitri_jdn`, `idul_adha_jdn`, `haul_jdn` helpers (tabular arithmetic). No prayer-time logic.
5. **Government date stub**: `indonesian_government_date()` returns `stub!("indonesian_government_date: requires Kemenag rukyat/hisab data; tabular approximation only. See https://sihat.kemenag.go.id")`.
6. **Traits**: Implement `CalendarDate`/`CalendarMetadata` using `calendar-core`. Add metadata for epoch JDN, sources list, `reference_sources()` exposure.
7. **Docs**: crate-level `//!` clarifying independent reimplementation, no misykat dependency, Apache-2.0 compatibility.

#### Testing matrix

- Anchor conversions: verify `hijri_to_jdn(1,1,1)=1948439`, `(...1043,1,1)=2317690`, `(...1355,1,1)=2428252`, `(...1446,1,1)=2460494`.
- Holiday spot checks: `idul_fitri_jdn(y) == hijri_to_jdn(y, 10, 1)`, `maulid_jdn(1446) == hijri_to_jdn(1446, 3, 12)`.
- Pasaran: `HijriDay::from_jdn(2317690)?.pasaran == Pasaran::Legi` (Jumat Legi).
- Property test: 1000 random JDNs within 1–1600 AH round-trip through `jdn_to_hijri`/`hijri_to_jdn`.
- Build-verification: `cargo build/test -p hijriyah --no-default-features` and `--target wasm32-unknown-unknown --no-default-features`.

Prayer-time computation, rukyat modeling, and any dependency on `misykat`/`praytime-rs` remain out of scope.

---

### `chinese-nusantara`

**Thin wrapper over `nongli` crate (v0.4.1, `github.com/supertsy5/nongli`, 7,639 total
downloads as of March 2026).**

Note: `nongli` depends on `chrono` with `std` features. This crate is therefore **`std`-only**
and must declare this per Fix 4. The `no_std` requirement does not apply here.

Add Peranakan (Chinese-Indonesian) context:
- **Cap Go Meh** (15th day of 1st lunar month) — major Nusantara celebration
- **Imlek** (Tahun Baru Imlek) — Indonesian national holiday since 2003
- **Shio** (zodiac animal) — 12-year cycle; Indonesian-language names
- **Weton Tionghoa**: intersection with Javanese Pasaran (in Peranakan practice)
- Regional variants: note Singkawang (Kalimantan Barat) Cap Go Meh customs differ
  from Solo and Semarang Peranakan traditions in auspiciousness interpretation

---

### `batak`

Batak traditional calendar is a lunar-star system — months counted by lunar phase,
years determined by the Constellation of Orion and Scorpius within the new moon phase,
yielding 12-month years and 13-month leap years.

**Six Batak sub-groups have variants**: Toba, Karo, Simalungun, Pakpak/Dairi, Angkola,
Mandailing. Core system is shared; variant divergences must be documented.

Implement:
- **Porhalaan**: the cylinder/book calendar artifact system — 12 months mapped to
  lunar phases, with intercalary month rules keyed to Orion (Bintang Tiga / Waluku)
  and Scorpius visibility
- **Hara** (day names): Toba system has 30-day named cycle
- **Datu system**: priest-computed auspiciousness for warding, healing, ritual timing
- Month names (Toba): Sipaha Sada, Sipaha Dua, Sipaha Tolu, Sipaha Opat, Sipaha Lima,
  Sipaha Onom, Sipaha Pitu, Sipaha Ualu, Sipaha Sia, Sipaha Sampulu, Hapungan, Hurung
  (intercalary: Ihuthon)

Primary source: "A Lunar-Star Calendar: Inquiry to the Traditional Batak Calendar" —
preprints.org/manuscript/202404.0235 (2024, peer-reviewed preprint)
Secondary: Schreiner, *Adat und Evangelium* (1972) — Porhalaan documentation

**Implementation constraint**: Because Porhalaan leap year depends on heliacal observation
(Orion/Scorpius), a purely algorithmic implementation requires the observer's latitude
(Lake Toba: ~2.6°N). Expose `from_astronomical(jdn, lat, lon)` and a `tabular` fallback.

---

### `sunda`

Sundanese calendar is closely related to Javanese but predates the Sultan Agung reform:
- **Kala Sunda**: pre-Islamic Sundanese Saka-derived system
- **Pranatamangsa Sunda**: 12 agricultural seasons (parallel to Javanese Pranata Masa
  but with different month names and slightly different solar epoch alignment)
- **Sunda Wiwitan** ceremonial calendar: used by Baduy (Kanekes) community — strictly
  oral tradition with no published algorithm; implement as `stub!()` with citation
- Naga Tahun cycle (year-direction taboo system)

---

### `bugis`

- **Bugis lunar calendar**: Islamic-influenced but retains pre-Islamic 12-month names
  (Rajab Bugis, etc.) with local intercalation rules
- **Hari Tudang Sipulung**: annual farming consultation ritual — timing computed from
  Pleiades (Bintang Kartika) visibility
- **Siri' timing**: certain adat obligations computed from lunar phase
- Note: Bugis calendar practices are partially documented in Pelras, *The Bugis* (1996)

---

### `sasak`

- **Sasak Rowot calendar**: new year determined by Pleiades (Bintang Rowot) first
  appearance above eastern horizon — astronomical computation required (same approach
  as Batak: observer-latitude-dependent)
- **Bau Nyale**: annual sea worm festival — 10th month, specific lunar day
- Month names: 12 months, Sasak-language
- Source: Taufiq, et al. — documentation via Lombok traditional astronomy communities

---

### `dayak`

- **Kaharingan agricultural calendar**: 12-month cycle keyed to Pleiades visibility
  and rice-cultivation phases (land clearing → planting → harvest)
- **Tewah** ritual timing: based on lunar phase and agricultural cycle
- Sub-group variants: Ngaju, Iban, Kenyah, Kayan, Murut each have dialect variants
  of month names; core cycle is shared
- Primary source: Schärer, *Ngaju Religion* (1963); Miles, *Cutlass and Crescent Moon*

---

### `toraja`

- **Toraja ritual calendar**: primarily organized around **Rambu Solo'** (death rituals)
  and **Rambu Tuka'** (life/prosperity rituals) — the two poles of the Toraja cosmos
- Timing of **Ma'nene'** (ancestral corpse-cleaning ceremony): August cycle, keyed to
  post-harvest period
- Month system: 12 lunar months; Toraja-language names
- Agricultural phases: rice cultivation tied to lunar cycle
- Source: Nooy-Palm, *The Sa'dan Toraja* Vol. 1 (1979)

---

### `tengger`

- **Tengger calendar**: closest living relative of pre-Islamic Javanese Hindu calendar;
  Tengger people of Bromo area did not convert to Islam, retaining Saka-based system
- **Kasada** ceremony: 14th day of Kasada month (12th month), pilgrimage to Bromo crater
  — this is the single most important computable date in the Tengger calendar
- **Unan-unan**: 5-year purification cycle. This is **algorithmically distinct** from the
  Javanese 8-year Windu and must not be conflated with it.
- Source: Hefner, *Hindu Javanese: Tengger Tradition and Islam* (1985)

---

### `minangkabau`

- **Minangkabau calendar**: primarily Islamic (Hijriyah) with local agricultural overlay
- **Turun ka sawah** (rice planting season): computed from combination of lunar month
  and Pleiades visibility
- **Hari Raya Adat**: distinct from Islamic Eid — local ceremonial new year
- Matrilineal clan cycle: time-based obligations (not strictly a calendar but computable)
- Source: Kato, *Matriliny and Migration* (1982)

---

### `dewasa-engine`

Cross-calendar auspiciousness correlator. Takes a Gregorian date and returns a unified
structure containing all active calendar representations plus cross-system analysis.

This crate is **`std`-only** (uses `HashMap`) and is explicitly exempt from the `no_std`
requirement. Declare `std` as a hard dependency in `Cargo.toml`.

```rust
pub struct NusantaraDay {
    pub gregorian: (i32, u8, u8),
    pub jdn: i64,

    // Always computed
    pub balinese: Option<balinese_calendar::BalineseDay>,
    pub javanese: Option<jawa::JavaneseDay>,
    pub hijriyah: Option<hijriyah::HijriDay>,
    pub chinese: Option<chinese_nusantara::ChineseDay>,

    // Computed if feature-flagged
    pub batak: Option<batak::BatakDay>,
    pub sunda: Option<sunda::SundaDay>,
    // ... etc

    pub cross_auspiciousness: HashMap<Activity, CrossCalendarVerdict>,
}

pub struct CrossCalendarVerdict {
    pub overall: AuspiciousnessLevel,
    pub by_calendar: HashMap<&'static str, AuspiciousnessLevel>,
    pub consensus_notes: Vec<&'static str>,
    pub conflicts: Vec<CalendarConflict>, // e.g. auspicious in Bali, inauspicious in Jawa
}
```

---

## Technical Constraints

| Crate | `no_std + alloc` | `std` required | Reason |
|---|---|---|---|
| `calendar-core` | ✅ required | — | trait definitions, JDN math |
| [`balinese-calendar`](https://github.com/SHA888/balinese-calendar) | ✅ required | — | pure arithmetic (crate: https://crates.io/crates/balinese-calendar) |
| `jawa` | ✅ required | — | pure arithmetic |
| `hijriyah` | ✅ (Option A) | if Option B/C | depends on license choice |
| `chinese-nusantara` | ❌ | ✅ | `nongli` → `chrono` requires `std` |
| `batak` | ✅ (tabular feature) | `astronomical` feature | astronomical feature uses floating-point |
| `sunda`, `tengger`, `bugis`, `sasak`, `dayak`, `toraja`, `minangkabau` | ✅ | — | pure arithmetic / stubs |
| `dewasa-engine` | ❌ | ✅ | `HashMap` aggregation |

Additional constraints applying to all crates:
- Rust edition 2021, MSRV 1.75+
- All static lookup tables as `const` — no runtime heap allocation for data
- WASM32 target must compile (verified via `cargo build --target wasm32-unknown-unknown`)
- Each crate independently publishable to crates.io
- Feature flags: `serde`, `wasm`, `astronomical` (for observation-dependent calendars)
- Every calendar crate implements `CalendarMetadata` from `calendar-core`
- Where algorithm is unknown/unconfirmed, use `CalendarError::NotImplemented` via `stub!()` —
  never silently fabricate data
- Dependency policy: `misykat` is GPL-3.0 — see `hijriyah` section for resolution;
  `nongli` v0.4.1 is the Chinese calendar dependency; audit license before publishing

---

## Build Order (strict — each depends on previous)

1. `calendar-core` — traits, JDN math, error types, `stub!()` macro
2. [`balinese-calendar`](https://github.com/SHA888/balinese-calendar) (expansion from Prompt 1; published at https://crates.io/crates/balinese-calendar) + `jawa` + `hijriyah` + `chinese-nusantara`
3. `batak` (most academically documented after Bali/Jawa)
4. `sunda`, `tengger` (algorithmically close to Jawa/Bali)
5. `bugis`, `sasak`, `dayak` (observation-dependent, partial stubs acceptable)
6. `toraja`, `minangkabau` (Islamic overlay + partial stubs)
7. `dewasa-engine` (depends on all above)

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
/// - [Citable reference 1]
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
