# TODO — `nusantara-calendar`

Organized by SDLC phase within each SemVer release milestone.
Each release is a Git tag on `main`. All items are checklist tasks.

**Legend:**
- `[arch]` — architecture / design decision
- `[impl]` — implementation
- `[test]` — testing
- `[doc]`  — documentation
- `[ci]`   — CI/CD / tooling
- `[rel]`  — release process

---

## Phase 0 — Repository & Workspace Bootstrap

> No version tag. Prerequisite for all subsequent milestones.

### 0.1 Repository Setup
- [ ] `[ci]` Initialize Git repository with `main` as default branch
- [ ] `[ci]` Add `.gitignore` (Rust standard: `target/`, `Cargo.lock` for libraries)
- [ ] `[ci]` Add `rust-toolchain.toml` pinning stable channel, MSRV 1.75
- [ ] `[ci]` Add `.cargo/config.toml` with `[alias]` for common workspace commands
- [ ] `[doc]` Add `LICENSE-MIT` and `LICENSE-APACHE` files at workspace root
- [ ] `[doc]` Add `CONTRIBUTING.md` with the citation-required policy
- [ ] `[doc]` Add `CHANGELOG.md` (Keep a Changelog format)
- [ ] `[doc]` Add `ARCHITECTURE.md` (this document's companion)
- [ ] `[doc]` Add `README.md` at workspace root

### 0.2 Workspace Cargo.toml
- [ ] `[impl]` Create `Cargo.toml` workspace root listing all member crates
- [ ] `[impl]` Add `[workspace.dependencies]` table with pinned shared deps
  - `thiserror = "2"` (latest stable)
  - `serde = { version = "1", features = ["derive"], optional = true }`
  - `wasm-bindgen = { version = "0.2", optional = true }`
  - `libm = { version = "0.2", optional = true }`
- [ ] `[impl]` Add `[profile.release]` with `lto = true`, `opt-level = "s"` for WASM size
- [ ] `[impl]` Add `[workspace.metadata.release]` for `cargo-release` config

### 0.3 CI Pipeline (GitHub Actions)
- [ ] `[ci]` Add `ci.yml`: `cargo check`, `cargo clippy`, `cargo test` on stable + MSRV 1.75
- [ ] `[ci]` Add WASM compilation check job:
  `cargo build --target wasm32-unknown-unknown -p calendar-core --no-default-features`
- [ ] `[ci]` Add `cargo test --no-default-features` job (verifies `no_std` compilability)
- [ ] `[ci]` Add `cargo deny check` for license auditing (catches GPL transitive deps)
- [ ] `[ci]` Add `deny.toml` with `deny = ["GPL-3.0"]` for workspace-level license policy
- [ ] `[ci]` Add `cargo doc --no-deps` build check (no broken intra-doc links)
- [ ] `[ci]` Configure Dependabot for `Cargo.toml` dependency updates

---

## v0.1.0 — `calendar-core`

> **Scope:** Shared traits, JDN math, error types, `stub!()` macro. No calendar logic.
> **Tag:** `calendar-core-v0.1.0`

### Design
- [ ] `[arch]` Confirm `i64` for JDN, `u32` for cycle-year fields, `u8` for sub-year positions
- [ ] `[arch]` Confirm `CalendarDate`, `CalendarMetadata`, `HasAuspiciousness` trait signatures
- [ ] `[arch]` Confirm `Activity` and `AuspiciousnessLevel` variants with `#[non_exhaustive]`
- [ ] `[arch]` Decide `CalendarError` string type: `String` (alloc) vs `&'static str` (no-alloc)
      → Decision: `String` (alloc); all `no_std` targets have `alloc` available

### Implementation
- [ ] `[impl]` Create `crates/calendar-core/` with `Cargo.toml`, `src/lib.rs`
- [ ] `[impl]` Implement `gregorian_to_jdn(y: i32, m: u8, d: u8) -> i64` as `pub const fn`
      (Meeus, *Astronomical Algorithms*, Ch. 7)
- [ ] `[impl]` Implement `jdn_to_gregorian(jdn: i64) -> (i32, u8, u8)` as `pub const fn`
- [ ] `[impl]` Define `CalendarDate` trait with default `from_gregorian` / `to_gregorian`
- [ ] `[impl]` Define `CalendarMetadata` trait
- [ ] `[impl]` Define `HasAuspiciousness` trait
- [ ] `[impl]` Define `Activity` enum (`#[non_exhaustive]`)
- [ ] `[impl]` Define `AuspiciousnessLevel` enum (`#[non_exhaustive]`)
- [ ] `[impl]` Define `CalendarError` with `thiserror`; `OutOfRange`, `NotImplemented`, `Ambiguous`
- [ ] `[impl]` Implement `stub!()` macro; export with `#[macro_export]`
- [ ] `[impl]` Add `#![no_std]` + `extern crate alloc` with `std` feature gate

### Testing
- [ ] `[test]` Unit test: `gregorian_to_jdn(1582, 10, 15) == 2299161` (reform anchor)
- [ ] `[test]` Unit test: `gregorian_to_jdn(1633, 7, 8) == 2317690` (Sultan Agung epoch)
- [ ] `[test]` Round-trip property test: 1000 random JDNs round-trip through `jdn_to_gregorian` → `gregorian_to_jdn`
- [ ] `[test]` Verify `stub!()` returns `Err(CalendarError::NotImplemented(_))`
- [ ] `[test]` `cargo test --no-default-features` passes (no_std)
- [ ] `[test]` WASM build: `cargo build --target wasm32-unknown-unknown -p calendar-core --no-default-features`

### Documentation
- [ ] `[doc]` Full rustdoc on all public items
- [ ] `[doc]` Crate-level `//!` doc with JDN pivot explanation and Meeus citation
- [ ] `[doc]` `reference_sources()` contract note in `CalendarMetadata` doc

### Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.1.0]` section
- [ ] `[rel]` `cargo publish -p calendar-core --dry-run`
- [ ] `[rel]` Tag `calendar-core-v0.1.0`
- [ ] `[rel]` `cargo publish -p calendar-core`

---

## v0.2.0 — Layer 2 Crates

> **Scope:** `balinese-calendar`, `jawa`, `hijriyah`, `chinese-nusantara`.
> All depend on `calendar-core` v0.1. Built concurrently, tagged together.
> **Tags:** `balinese-calendar-v0.2.0`, `jawa-v0.2.0`, `hijriyah-v0.2.0`, `chinese-nusantara-v0.2.0`

---

### 2.1 `balinese-calendar`

#### Design
- [ ] `[arch]` Audit existing `balinese-calendar` v0.1.1 crate (authored by Kresna)
      against `CalendarDate` + `CalendarMetadata` trait signatures from v0.1.0
- [ ] `[arch]` Plan migration: add trait impls without breaking existing public API
- [ ] `[arch]` Confirm supported year range (document in rustdoc and return `OutOfRange` at boundaries)

#### Implementation
- [ ] `[impl]` Add `calendar-core` dependency
- [ ] `[impl]` Implement `CalendarDate` for `BalineseDay`
- [ ] `[impl]` Implement `CalendarMetadata` for `BalineseDay` with Saka epoch JDN and sources
- [ ] `[impl]` Implement `HasAuspiciousness` for known auspiciousness rules (Dewasa Ayu etc.)
- [ ] `[impl]` Gate `no_std` correctly; add `std` feature flag

#### Testing
- [ ] `[test]` Round-trip test: 500 random JDNs for `BalineseDay`
- [ ] `[test]` Known-date regression: verify Nyepi 2026 falls on correct Saka day
- [ ] `[test]` WASM build check
- [ ] `[test]` `--no-default-features` build check

---

### 2.2 `jawa`

#### Design
- [ ] `[arch]` Confirm no Rust Javanese crate exists on crates.io (last checked March 2026)
- [ ] `[arch]` Map all required cycles to data types:
      Wetonan → `(SaptawaraPos, PasaranPos)`, Pawukon → `WukuPos`, Windu → `WinduYear` enum,
      Kurup → `KurupRecord`, Pranata Masa → `PranataMasaPos`
- [ ] `[arch]` Specify supported year range: AJ 1555–2474 (Gregorian 1633–2169, spans 2 kurups)

#### Implementation
- [ ] `[impl]` Create `crates/jawa/` skeleton
- [ ] `[impl]` Define `SULTAN_AGUNG_EPOCH_JDN: i64 = 2317690` as `pub const`
      (cite: Beauducel & Karjanto 2020, arXiv:2012.10064)
- [ ] `[impl]` Implement `WinduYear` enum with `from_aj(u32)` and `is_leap()` methods
- [ ] `[impl]` Implement `const` Pasaran names array (5 entries, ngoko + krama)
- [ ] `[impl]` Implement `const` Saptawara names array (7 entries) with neptu values
- [ ] `[impl]` Implement `const` Wuku names array (30 entries, Javanese-language)
- [ ] `[impl]` Implement `const` Pranata Masa names array (12 entries) with solar epoch offsets
- [ ] `[impl]` Implement `const` Dina Mulya table
- [ ] `[impl]` Implement Wetonan computation from JDN (Beauducel-Karjanto congruence formula)
- [ ] `[impl]` Implement Pawukon (210-day) position from JDN (D-R Ch. 10 algorithm)
- [ ] `[impl]` Implement Wulan (lunar month) arithmetic from JDN using tabular Hijri cycle
- [ ] `[impl]` Implement `WinduYear::from_aj()` and full `JavaneseYear` from AJ number
- [ ] `[impl]` Implement Pranata Masa from JDN (solar position relative to ~April 22 epoch)
- [ ] `[impl]` Implement `KurupRecord` (current: Alip Selasa Pon, 1936-03-24 → 2052-08-25)
- [ ] `[impl]` Stub supra-windu group names with citation (Danudji 2006)
- [ ] `[impl]` Implement `CalendarDate` for `JavaneseDay`
- [ ] `[impl]` Implement `CalendarMetadata` for `JavaneseDay`
- [ ] `[impl]` Gate `no_std` correctly

#### Testing
- [ ] `[test]` Known anchor: JDN 2317690 → 1 Sura 1555 AJ, Jumat Legi, wuku Sinta pos 1
- [ ] `[test]` Known anchor: 1945-08-17 (Proklamasi) → Jumat Legi (verify historical record)
- [ ] `[test]` Kurup boundary: 1936-03-24 → Selasa Pon, Alip year
- [ ] `[test]` Windu year: `WinduYear::from_aj(1959) == WinduYear::Wawu`
- [ ] `[test]` Round-trip: 500 random JDNs within AJ 1555–2474
- [ ] `[test]` `--no-default-features` + WASM build check

---

### 2.3 `hijriyah`

> Detailed execution plan tracked at `~/.windsurf/plans/hijriyah-implementation-a99a87.md`.

#### Design
- [ ] `[arch]` Confirm GPL-3.0-only exclusion (misykat) and record Option A decision in `hijriyah/DECISION.md`.
- [ ] `[arch]` Define crate structure (`Cargo.toml`, `src/`, `tests/`, `DECISION.md`, `SOURCES.md`).
- [ ] `[arch]` Specify arithmetic sources (Dershowitz-Reingold Ch. 6, Meeus Ch. 9) and cite in rustdoc.
- [ ] `[arch]` Declare `tabular_date()` vs `indonesian_government_date()` behavior and supported Hijri range (≥1–1600 AH).

#### Implementation
- [ ] `[impl]` Create crate skeleton (`Cargo.toml`, `src/lib.rs`, `arithmetic.rs`, `types.rs`, `holidays.rs`, `metadata.rs`, `tests/anchors.rs`).
- [ ] `[impl]` Add `#![cfg_attr(not(feature = "std"), no_std)]` + `extern crate alloc`; wire features (`std`, `serde`, `wasm`).
- [ ] `[impl]` Implement `hijri_to_jdn` and `jdn_to_hijri` per D-R Eq. 6.2–6.3 (tabular, Thursday epoch, JDN 1948439 start).
- [ ] `[impl]` Implement leap-year logic (years 2,5,7,10,13,16,18,21,24,26,29 in each 30-year cycle) and expose `HijriDay::is_leap_year`.
- [ ] `[impl]` Build `HijriDay` struct with month metadata (Arabic + Indonesian names), `day_of_year`, `pasaran` field.
- [ ] `[impl]` Implement Pasaran calculation `(jdn + 2) % 5` as standalone helper (no dependency on `jawa`).
- [ ] `[impl]` Implement holiday helpers: `maulid_jdn`, `isra_miraj_jdn`, `idul_fitri_jdn`, `idul_adha_jdn`, `haul_jdn`.
- [ ] `[impl]` Provide `tabular_date()` and stub `indonesian_government_date()` with `stub!()` message referencing Kemenag data need.
- [ ] `[impl]` Document algorithm choice and record exclusion rationale in `DECISION.md`; cite references in `SOURCES.md`.
- [ ] `[impl]` Implement `CalendarDate` + `CalendarMetadata` traits for `HijriDay` (no_std ready).

#### Testing
- [ ] `[test]` Anchor JDNs: 1 Muharram 1 AH (1948439), 1043 AH (2317690), 1355 AH (2428252), 1446 AH (2460494).
- [ ] `[test]` Holiday equality: ensure `idul_fitri_jdn(y) == hijri_to_jdn(y, 10, 1)` etc.
- [ ] `[test]` Pasaran check: `HijriDay::from_jdn(2317690)?.pasaran == Pasaran::Legi` (Jumat Legi).
- [ ] `[test]` Round-trip property: 1000 random JDNs within 1–1600 AH.
- [ ] `[test]` no_std + WASM builds: `cargo build/test -p hijriyah --no-default-features` and `--target wasm32-unknown-unknown`.

---

### 2.4 `chinese-nusantara`

#### Design
- [ ] `[arch]` Audit `nongli` v0.4.1 API: `ChineseDate`, `SolarTerm`; confirm no JDN interface
- [ ] `[arch]` Design bridge: `gregorian_to_jdn` → `nongli::ChineseDate::from_gregorian(NaiveDate)`
      Note: `NaiveDate` is a `chrono` type; bridge via `jdn_to_gregorian` → `NaiveDate::from_ymd_opt`
- [ ] `[arch]` Document that this crate is `std`-only in `Cargo.toml` and rustdoc

#### Implementation
- [ ] `[impl]` Create `crates/chinese-nusantara/` skeleton
- [ ] `[impl]` Add `nongli = "0.4.1"` dependency
- [ ] `[impl]` Implement `ChineseNusantaraDay` wrapping `nongli::ChineseDate`
- [ ] `[impl]` Implement `from_jdn` bridge via `jdn_to_gregorian` → `chrono::NaiveDate`
- [ ] `[impl]` Add `Shio` (zodiac) enum with 12 variants + Indonesian names
- [ ] `[impl]` Implement `Shio::from_chinese_year(year: i64) -> Shio`
- [ ] `[impl]` Implement `cap_go_meh_jdn(chinese_year: i64) -> i64` (15th day, 1st month)
- [ ] `[impl]` Implement `imlek_jdn(chinese_year: i64) -> i64` (1st day, 1st month)
- [ ] `[impl]` Implement Weton Tionghoa (Pasaran intersection; same formula as hijriyah)
- [ ] `[impl]` Document Singkawang vs Solo/Semarang Cap Go Meh variant in rustdoc
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`

#### Testing
- [ ] `[test]` Imlek 2026: verify Chinese New Year 1st day maps to 2026-01-29 Gregorian
- [ ] `[test]` Cap Go Meh 2026: verify 15th day 1st month maps to 2026-02-12 Gregorian
- [ ] `[test]` Shio: verify 2026 is year of the Horse (Kuda)
- [ ] `[test]` Round-trip: 200 dates (nongli range: 1900–2100)

### v0.2.0 Release
- [ ] `[rel]` Integration test: `dewasa-engine` smoke test with all 4 crates (pre-release build)
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.2.0]` section for each crate
- [ ] `[rel]` Dry-run publish all 4 crates
- [ ] `[rel]` Tag and publish: `balinese-calendar-v0.2.0`, `jawa-v0.2.0`, `hijriyah-v0.2.0`, `chinese-nusantara-v0.2.0`

---

## v0.3.0 — `batak`

> **Scope:** Batak Porhalaan with Toba, Karo, Simalungun variants.
> **Tag:** `batak-v0.3.0`

### Design
- [ ] `[arch]` Read and summarize: "A Lunar-Star Calendar", preprints.org/manuscript/202404.0235 (2024)
- [ ] `[arch]` Map Toba Porhalaan 12-month system to data types; identify intercalation rule
- [ ] `[arch]` Specify `tabular` feature (predictive Orion-based table) vs `astronomical` (runtime calc)
- [ ] `[arch]` Document sub-group variant divergences: Karo, Simalungun vs Toba core

### Implementation
- [ ] `[impl]` Create `crates/batak/` skeleton
- [ ] `[impl]` Implement `const` Toba month names array (12 + 1 intercalary: Ihuthon)
- [ ] `[impl]` Implement `const` Hara day-name cycle (30 entries)
- [ ] `[impl]` Implement tabular Porhalaan: JDN → lunar month via Islamic-style 12/13-month cycle
- [ ] `[impl]` Implement `BatakDay::from_jdn_tabular()`
- [ ] `[impl]` Implement `BatakDay::from_jdn_astronomical(jdn, lat, lon)` (feature-gated)
      using `libm` for heliacal angle computation at Lake Toba (~2.6°N)
- [ ] `[impl]` Stub Karo variant month names with citation
- [ ] `[impl]` Stub Simalungun variant month names with citation
- [ ] `[impl]` Stub Datu auspiciousness system with citation (Schreiner 1972)
- [ ] `[impl]` Implement `CalendarDate` (calls `from_jdn_tabular`), `CalendarMetadata`

### Testing
- [ ] `[test]` Tabular: verify known Toba festival dates against preprint data
- [ ] `[test]` Round-trip: 200 JDNs, tabular mode
- [ ] `[test]` Astronomical: verify Orion first-rise JDN at lat=2.6°N for 2025 matches
      published astronomical data (use USNO or JPL Horizons reference)
- [ ] `[test]` `--no-default-features` + `--features astronomical` builds both pass

### Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.3.0]`
- [ ] `[rel]` Tag and publish `batak-v0.3.0`

---

## v0.4.0 — `sunda` + `tengger`

> **Tag:** `sunda-v0.4.0`, `tengger-v0.4.0`

### 4.1 `sunda`

- [ ] `[arch]` Research Kala Sunda Saka epoch and relationship to Balinese Saka
- [ ] `[arch]` Document Pranatamangsa Sunda solar epoch difference vs Javanese
- [ ] `[impl]` Implement Kala Sunda (Saka-derived arithmetic, solar year)
- [ ] `[impl]` Implement Pranatamangsa Sunda (12 seasons, Sundanese month names)
- [ ] `[impl]` Implement Naga Tahun cycle (8-direction year taboo)
- [ ] `[impl]` Stub Sunda Wiwitan (Baduy oral tradition) with citation: Garna (1993)
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Pranatamangsa Sunda: verify season 1 (Kasa) start date aligns with cited source
- [ ] `[test]` Round-trip: 200 JDNs

### 4.2 `tengger`

- [ ] `[arch]` Read Hefner (1985) for Kasada computation rules; identify Saka base
- [ ] `[arch]` Confirm Unan-unan 5-year cycle is distinct from Javanese Windu
- [ ] `[impl]` Implement Tengger Saka calendar (Saka arithmetic, Hindu-derived)
- [ ] `[impl]` Implement `kasada_jdn(tengger_year: u32) -> i64`
      (14th day, 12th month Kasada = annual pilgrimage to Bromo)
- [ ] `[impl]` Implement Unan-unan 5-year purification cycle
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Kasada 2026: verify JDN maps to the correct Gregorian date
      (cross-reference with Bromo ceremony news sources)
- [ ] `[test]` Round-trip: 200 JDNs

### v0.4.0 Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.4.0]`
- [ ] `[rel]` Tag and publish `sunda-v0.4.0`, `tengger-v0.4.0`

---

## v0.5.0 — `bugis` + `sasak` + `dayak`

> **Scope:** Observation-dependent crates; partial stubs accepted.
> **Tag:** `bugis-v0.5.0`, `sasak-v0.5.0`, `dayak-v0.5.0`

### 5.1 `bugis`

- [ ] `[arch]` Read Pelras, *The Bugis* (1996) — document calendar sections
- [ ] `[impl]` Implement 12-month Bugis lunar calendar (Islamic-influenced)
- [ ] `[impl]` Implement `const` pre-Islamic Bugis month names (12 entries)
- [ ] `[impl]` Implement Siri' timing (lunar phase lookup)
- [ ] `[impl]` Implement `tudang_sipulung_jdn(year)` stub: Pleiades first rise,
      cite astronomical source; provide tabular approximation for ~3°S latitude
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Known anchor: verify Idul Fitri 1446 AH Bugis date alignment with Hijriyah
- [ ] `[test]` `--no-default-features` build

### 5.2 `sasak`

- [ ] `[arch]` Read Taufiq et al. — document Rowot new year and Bau Nyale timing
- [ ] `[impl]` Implement 12-month Sasak lunar calendar with Sasak-language month names
- [ ] `[impl]` Implement `rowot_new_year_jdn(year)`:
      Pleiades first rise above eastern horizon, Lombok (~8.6°S); tabular fallback
- [ ] `[impl]` Implement `bau_nyale_jdn(year)`: 10th month, specific lunar day lookup
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Bau Nyale 2025/2026: verify Gregorian date against published festival announcements
- [ ] `[test]` Round-trip: 150 JDNs

### 5.3 `dayak`

- [ ] `[arch]` Read Schärer (1963) *Ngaju Religion* — document Kaharingan calendar phases
- [ ] `[impl]` Implement 12-month Kaharingan agricultural cycle (Pleiades-keyed)
- [ ] `[impl]` Implement `const` Ngaju month names (12 entries); stub Kayan/Kenyah variants
- [ ] `[impl]` Implement Tewah ritual timing (lunar phase + agricultural cycle intersection)
- [ ] `[impl]` Stub Iban, Kenyah, Murut variant month names with citations
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Month boundary: verify Dayak new year approximate JDN vs Pleiades season

### v0.5.0 Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.5.0]`
- [ ] `[rel]` Tag and publish all three crates

---

## v0.6.0 — `toraja` + `minangkabau`

> **Tag:** `toraja-v0.6.0`, `minangkabau-v0.6.0`

### 6.1 `toraja`

- [ ] `[arch]` Read Nooy-Palm (1979) *The Sa'dan Toraja* Vol. 1 — extract month names and
      Rambu Solo' / Rambu Tuka' timing rules
- [ ] `[impl]` Implement 12-month Toraja lunar calendar with Toraja-language month names
- [ ] `[impl]` Implement `rambu_solo_season_jdn(year)` (post-harvest period, approx. June–Oct)
- [ ] `[impl]` Implement `manene_jdn(year)` (August cycle; keyed to post-harvest lunar phase)
- [ ] `[impl]` Implement Rambu Tuka' season (planting/prosperity; inverse of Rambu Solo')
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Ma'nene' 2024/2025: verify approximate date against documented ceremony records

### 6.2 `minangkabau`

- [ ] `[arch]` Read Kato (1982) *Matriliny and Migration* — document adat calendar obligations
- [ ] `[impl]` Implement Hijriyah-overlay base (re-use `jdn_to_hijri` from `hijriyah`; no import)
- [ ] `[impl]` Implement `turun_ka_sawah_jdn(year)`:
      Pleiades visibility + lunar month conjunction, tabular fallback for 0° latitude
- [ ] `[impl]` Implement `hari_raya_adat_jdn(year)` stub with citation
- [ ] `[impl]` Implement matrilineal clan cycle time-obligation lookup (stub, adat-dependent)
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Verify Turun ka Sawah approximate season against Padang agricultural calendar

### v0.6.0 Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.6.0]`
- [ ] `[rel]` Tag and publish both crates

---

## v0.7.0 — `dewasa-engine`

> **Scope:** Cross-calendar auspiciousness correlator. Depends on all prior crates.
> **Tag:** `dewasa-engine-v0.7.0`

### Design
- [ ] `[arch]` Finalize `NusantaraDay` struct field list; confirm `Option<_>` for all ethnic crates
- [ ] `[arch]` Define `CrossCalendarVerdict` consensus algorithm (most cautious non-Unknown wins)
- [ ] `[arch]` Define `CalendarConflict` struct: which calendars, what levels, what dates
- [ ] `[arch]` Decide feature flags for ethnic crates in `dewasa-engine`:
      `features = ["batak", "sunda", "tengger", "bugis", "sasak", "dayak", "toraja", "minangkabau"]`
      Default: only the 4 core crates always included

### Implementation
- [ ] `[impl]` Create `crates/dewasa-engine/` with `std`-only declaration
- [ ] `[impl]` Implement `NusantaraDay::from_gregorian(y, m, d)` calling all enabled crates
- [ ] `[impl]` Implement `NusantaraDay::from_jdn(jdn: i64)` as primary constructor
- [ ] `[impl]` Implement `CrossCalendarVerdict::compute(verdicts: &[(name, AuspiciousnessLevel)])`
- [ ] `[impl]` Implement conflict detection: opposite sides of Neutral across 2+ calendars
- [ ] `[impl]` Implement `overall` consensus: most cautious non-Unknown level
- [ ] `[impl]` Add `serde` feature for full `NusantaraDay` serialization (JSON-ready for API)

### Testing
- [ ] `[test]` Smoke test: `NusantaraDay::from_gregorian(2026, 3, 21)` returns valid struct
- [ ] `[test]` Conflict detection: construct a day with known opposing auspiciousness values
- [ ] `[test]` Serialization: `serde_json::to_string(&day)` round-trips correctly
- [ ] `[test]` Feature-gated ethnic crates: build with/without `batak` feature

### Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.7.0]`
- [ ] `[rel]` Tag and publish `dewasa-engine-v0.7.0`

---

## v1.0.0 — API Stabilization

> **Scope:** Freeze public API. Resolve all stubs where algorithm is now available.
> Complete documentation. Production-ready for `dedauh.id`.
> **Tags:** All crates at `v1.0.0`

### Pre-release Audit
- [ ] `[arch]` Review all `stub!()` calls across all crates
      — for each: either resolve with a verified algorithm or confirm it remains a stub
- [ ] `[arch]` Confirm `#[non_exhaustive]` on all enums that may gain variants post-1.0
- [ ] `[arch]` Audit all `CalendarMetadata::reference_sources()` — no empty slices allowed
- [ ] `[arch]` Resolve `indonesian_government_date()` stub in `hijriyah`
      (requires Kemenag hisab algorithm source; stub if still unresolvable)
- [ ] `[arch]` Full license audit via `cargo deny check` — confirm no GPL transitive deps

### API Hardening
- [ ] `[impl]` Add `#[must_use]` to all `from_jdn` / `from_gregorian` return values
- [ ] `[impl]` Add `#[inline]` to hot-path cycle arithmetic functions
- [ ] `[impl]` Ensure all public types implement `Debug`, `Clone`, `PartialEq`
- [ ] `[impl]` Add `Hash` to all enums (required for use as `HashMap` keys)
- [ ] `[impl]` Verify `serde` round-trips for all public structs across all crates
- [ ] `[impl]` Verify `wasm-bindgen` exports compile and tree-shake correctly

### Documentation
- [ ] `[doc]` Complete rustdoc coverage: `cargo doc --no-deps` with zero warnings
- [ ] `[doc]` Add `# Examples` section to every public `fn`
- [ ] `[doc]` Add `# Panics` / `# Errors` sections where applicable
- [ ] `[doc]` Write crate-level `//!` narrative for all 14 crates
- [ ] `[doc]` Finalize `README.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`
- [ ] `[doc]` Write `SOURCES.md` at workspace root — master list of all cited references
- [ ] `[doc]` Write `STUBS.md` at workspace root — inventory of all `stub!()` calls with
      current status (pending / in-progress / will-not-implement + reason)

### Testing
- [ ] `[test]` Achieve ≥80% line coverage across all non-stub code
- [ ] `[test]` Add property-based tests (via `proptest`) for all `CalendarDate::from_jdn` impls
- [ ] `[test]` Add benchmark suite (`criterion`) for JDN pivot functions
- [ ] `[test]` Full WASM build matrix: all `no_std` crates on `wasm32-unknown-unknown`
- [ ] `[test]` MSRV check: `cargo +1.75 test --workspace`

### CI Additions for v1.0
- [ ] `[ci]` Add `cargo semver-checks` to CI (detects unintentional breaking changes)
- [ ] `[ci]` Add coverage report (via `cargo llvm-cov`) with ≥80% gate
- [ ] `[ci]` Publish docs to `docs.rs` and verify all intra-doc links resolve

### Release
- [ ] `[rel]` Write `CHANGELOG.md` — `## [1.0.0]` with migration notes from 0.x
- [ ] `[rel]` Bump all crate versions to `1.0.0`
- [ ] `[rel]` `cargo publish` in dependency order (core → layer 2 → ethnic → engine)
- [ ] `[rel]` Tag `v1.0.0` on `main`
- [ ] `[rel]` Create GitHub Release with changelog extract
- [ ] `[rel]` Announce on crates.io, lib.rs, and relevant Indonesian developer communities

---

## Post-1.0 Backlog

> These items are out of scope for v1.0 but are planned for future minor releases.

### v1.1 — Stub Resolution Pass
- [ ] `[impl]` Implement Kemenag hisab algorithm if official source is published
- [ ] `[impl]` Implement Sunda Wiwitan if algorithm is documented by Baduy community representatives
- [ ] `[impl]` Implement Karo / Simalungun Batak variants if sources become available
- [ ] `[impl]` Implement supra-windu group names (Sengara, Langkir, Kuntara, Adi) if Danudji 2006
      physical book is digitized and verified

### v1.2 — `astronomical` Feature Maturity
- [ ] `[impl]` Validate Batak astronomical Orion/Scorpius computation against USNO data for 2020–2030
- [ ] `[impl]` Validate Sasak Pleiades first-rise dates against published Lombok astronomy records
- [ ] `[impl]` Add `HeliacalEvent` type to `calendar-core` for sharing astronomical results
      across crates without coupling

### v1.3 — `dedauh.id` Integration Layer
- [ ] `[impl]` Design and publish `nusantara-calendar-wasm` meta-crate bundling all WASM exports
- [ ] `[impl]` Add npm package generation (`wasm-pack build`)
- [ ] `[impl]` Add TypeScript type definitions for WASM API surface
- [ ] `[impl]` Add REST-style JSON serialization helpers for SaaS API layer

### v1.4 — Additional Ethnic Calendars
- [ ] `[arch]` Research Acehnese calendar (Islamic overlay + pre-Islamic elements)
- [ ] `[arch]` Research Nias traditional calendar (South Nias lunar cycle)
- [ ] `[arch]` Research Ambonese / Maluku calendar systems
- [ ] `[arch]` Research Papuan (Dani, Asmat) agricultural cycle documentation
