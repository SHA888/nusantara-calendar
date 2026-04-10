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
- [x] `[ci]` Initialize Git repository with `main` as default branch
- [x] `[ci]` Add `.gitignore` (Rust standard: `target/`, `Cargo.lock` for libraries)
- [x] `[ci]` Add `rust-toolchain.toml` pinning stable channel, MSRV 1.80
- [x] `[ci]` Add `.cargo/config.toml` with `[alias]` for common workspace commands
- [x] `[doc]` Add `LICENSE-MIT` and `LICENSE-APACHE` files at workspace root
- [x] `[doc]` Add `CONTRIBUTING.md` with the citation-required policy
- [x] `[doc]` Add `CHANGELOG.md` (Keep a Changelog format)
- [x] `[doc]` Add `ARCHITECTURE.md` (this document's companion)
- [x] `[doc]` Add `README.md` at workspace root

### 0.2 Workspace Cargo.toml
- [x] `[impl]` Create `Cargo.toml` workspace root listing `calendar-core` and `nusantara-calendar`
- [x] `[impl]` Add `[workspace.dependencies]` table with pinned shared deps
  - `thiserror = "2"` (latest stable)
  - `serde = { version = "1", features = ["derive"], optional = true }`
  - `wasm-bindgen = { version = "0.2", optional = true }`
  - `libm = { version = "0.2", optional = true }`
- [x] `[impl]` Add `[profile.release]` with `lto = true`, `opt-level = "s"` for WASM size
- [x] `[impl]` Add `[workspace.metadata.release]` for `cargo-release` config

### 0.3 CI Pipeline (GitHub Actions)
- [x] `[ci]` Add `ci.yml`: `cargo check`, `cargo clippy`, `cargo test` on stable + MSRV 1.80
- [x] `[ci]` Add WASM compilation check job:
  `cargo build --target wasm32-unknown-unknown -p calendar-core --no-default-features`
- [x] `[ci]` Add `cargo test --no-default-features` job (verifies `no_std` compilability)
- [x] `[ci]` Add `cargo deny check` for license auditing (catches GPL transitive deps)
- [x] `[ci]` Add `deny.toml` with `deny = ["GPL-3.0"]` for workspace-level license policy
- [x] `[ci]` Add `cargo doc --no-deps` build check (no broken intra-doc links)
- [x] `[ci]` Configure Dependabot for `Cargo.toml` dependency updates

---

## v0.1.0 — `calendar-core`

> **Scope:** Shared traits, JDN math, error types, `stub!()` macro. No calendar logic.
> **Tag:** `calendar-core-v0.1.0`

### Design
- [x] `[arch]` Confirm `i64` for JDN, `u32` for cycle-year fields, `u8` for sub-year positions
- [x] `[arch]` Confirm `CalendarDate`, `CalendarMetadata`, `HasAuspiciousness` trait signatures
- [x] `[arch]` Confirm `Activity` and `AuspiciousnessLevel` variants with `#[non_exhaustive]`
- [x] `[arch]` Decide `CalendarError` string type: `String` (alloc) vs `&'static str` (no-alloc)
      → Decision: `String` (alloc); all `no_std` targets have `alloc` available

### Implementation
- [x] `[impl]` Create `crates/calendar-core/` with `Cargo.toml`, `src/lib.rs`
- [x] `[impl]` Implement `gregorian_to_jdn(year: i32, month: u8, day: u8) -> JDN`
      (Meeus, *Astronomical Algorithms*, Ch. 7)
- [x] `[impl]` Implement `jdn_to_gregorian(jdn: JDN) -> (i32, u8, u8)`
      (Fliegel & van Flandern 1968, USNO algorithm)
- [x] `[impl]` Define `CalendarDate` trait with `calendar_name()`, `validate_range()`,
      and default `from_gregorian` / `to_gregorian`
- [x] `[impl]` Define `CalendarMetadata` trait with `epoch()`, `cycle_length()`,
      `description()`, `cultural_origin()`
- [x] `[impl]` Define `HasAuspiciousness` trait with associated types
- [x] `[impl]` Define `Activity` enum (`#[non_exhaustive]`): Marriage, Building, Travel,
      Business, Agriculture, ReligiousCeremony, Naming, MovingHouse, Education, Medical, Custom
- [x] `[impl]` Define `AuspiciousnessLevel` enum (`#[non_exhaustive]`):
      VeryAuspicious, Auspicious, Neutral, Inauspicious, VeryInauspicious
- [x] `[impl]` Define `CalendarError`: OutOfRange, InvalidParameters, NotImplemented, ArithmeticError
- [x] `[impl]` Implement `stub!($msg:expr)` macro; export with `#[macro_export]`
- [x] `[impl]` Add `#![no_std]` + `extern crate alloc` with `std` feature gate

### Testing
- [x] `[test]` Unit test: `gregorian_to_jdn(1582, 10, 15) == 2299161` (reform anchor)
- [x] `[test]` Unit test: `gregorian_to_jdn(1633, 7, 8) == 2317690` (Sultan Agung epoch)
- [x] `[test]` Round-trip property test: 1000 random JDNs round-trip through `jdn_to_gregorian` → `gregorian_to_jdn`
- [x] `[test]` Verify `stub!()` returns `Err(CalendarError::NotImplemented(_))`
- [x] `[test]` `cargo test --no-default-features` passes (no_std)
- [x] `[test]` WASM build: `cargo build --target wasm32-unknown-unknown -p calendar-core --no-default-features`
- [x] `[test]` Fix JDN to Gregorian conversion using Fliegel & van Flandern algorithm (all 10 tests passing)

### Documentation
- [x] `[doc]` Full rustdoc on all public items
- [x] `[doc]` Crate-level `//!` doc with JDN pivot explanation and algorithm citations
- [x] `[doc]` `description()` / `cultural_origin()` contract note in `CalendarMetadata` doc

### Release
- [x] `[rel]` Update `CHANGELOG.md` — `## [0.1.0]` section

---

## v0.1.0 — `nusantara-calendar` Architecture & Balinese

> **Scope:** Workspace refactor to 2-crate structure; balinese module wrapping external crate.
> **Tag:** `nusantara-calendar-v0.1.0`

### `nusantara-calendar` Architecture Refactor

- [x] `[arch]` Confirm single-crate structure with feature-gated modules (not separate crates per calendar)
- [x] `[arch]` Implement feature flags for each calendar module in `Cargo.toml`
- [x] `[arch]` Keep `calendar-core` as separate published dependency
- [x] `[arch]` Plan wrapper approach for external crates (balinese-calendar)
- [x] `[impl]` Create `crates/nusantara-calendar/Cargo.toml` with all feature flags
- [x] `[impl]` Create `src/lib.rs` with feature-gated `mod` declarations and `calendar-core` re-exports
- [x] `[impl]` Create stub `mod.rs` files for all calendar modules (jawa through dewasa_engine)
- [x] `[impl]` Implement balinese wrapper around external `balinese-calendar` v0.2.0
- [x] `[impl]` Implement `calendar-core` traits on wrapped balinese types
- [x] `[test]` Verify balinese wrapper trait implementations
- [x] `[test]` Round-trip conversions with official crate
- [x] `[test]` Feature flag compilation tests
- [x] `[test]` Build checks for all feature combinations

---

### `balinese` module

- [x] `[arch]` Audit official `balinese-calendar` v0.2.0 crate API
- [x] `[arch]` Design wrapper that re-exports official types via `pub use`
- [x] `[arch]` Plan trait delegation to official crate methods
- [x] `[impl]` Implement `src/balinese/mod.rs` wrapper module
- [x] `[impl]` Implement `CalendarDate` for `BalineseDate` newtype
- [x] `[impl]` Implement `CalendarMetadata` for `BalineseDate`
- [x] `[impl]` Implement `HasAuspiciousness` for `BalineseDate`
- [x] `[impl]` Add `from_ymd`, `as_official`, `from_official` conversion methods
- [x] `[impl]` Implement `Deref<Target = OfficialBalineseDate>` and `From` conversion
- [x] `[test]` Verify all three trait implementations work correctly
- [x] `[test]` Test round-trip conversions via JDN
- [x] `[test]` Test official crate access through wrapper
- [x] `[test]` Validation and error handling tests (out-of-range years)

---

## v0.2.0 — Core Calendar Modules (jawa, hijriyah, chinese-nusantara)

> **Scope:** Three core supra-ethnic calendar modules implemented in `nusantara-calendar`.
> **Tag:** `nusantara-calendar-v0.2.0`

### `jawa` module

- [x] `[arch]` Confirm independent implementation — no Rust Javanese calendar crate exists on crates.io; existing implementations are MATLAB/Perl/C (`beaudu/weton`) and JS/TS (`kalenderjawa` org); primary source confirmed: Karjanto & Beauducel (2020), arXiv:2012.10064
- [x] `[arch]` Map all required cycles to data types:
      Wetonan → `(SaptawaraPos, PasaranPos)`, Pawukon → `WukuPos`, Windu → `WinduYear` enum,
      Kurup → `KurupRecord`, Pranata Masa → `PranataMasaPos`
      *Implemented in `src/jawa/mod.rs` with const arrays for names/neptu per ARCHITECTURE.md*
- [x] `[arch]` Specify supported year range: AJ 1555–2474 (Gregorian 1633–2169, spans 2 kurups)
      *Defined `AJ_MIN`, `AJ_MAX`, `JDN_MIN`, `JDN_MAX` constants*
- [x] `[impl]` Implement `src/jawa/mod.rs` within `nusantara-calendar`
      *All data types, const arrays, and stub `CalendarDate`/`CalendarMetadata` traits implemented*
- [x] `[impl]` Define `pub const SULTAN_AGUNG_EPOCH_JDN: i64 = 2317690`
      (cite: Karjanto & Beauducel 2020, arXiv:2012.10064)
- [x] `[impl]` Implement `WinduYear` enum with `from_aj(u32)` and `is_leap()` methods
- [x] `[impl]` Implement `const` Pasaran names array (5 entries, ngoko + krama)
- [x] `[impl]` Implement `const` Saptawara names array (7 entries) with neptu values
- [x] `[impl]` Implement `const` Wuku names array (30 entries, Javanese-language)
- [x] `[impl]` Implement `const` Pranata Masa names array (12 entries) with solar epoch offsets
- [x] `[impl]` Implement `const` Dina Mulya table
- [x] `[impl]` Implement Wetonan computation from JDN (Karjanto-Beauducel congruence formula)
      *Formulas: `pasaran = jdn.rem_euclid(5)`, `saptawara = jdn.rem_euclid(7)`*
- [x] `[impl]` Implement Pawukon (210-day) position from JDN (D-R Ch. 10 algorithm)
      *Formula: `wuku = ((jdn / 7) + 12).rem_euclid(30)` with offset for epoch alignment*
- [x] `[impl]` Implement Wulan (lunar month) arithmetic from JDN using tabular cycle
      *Iterative year/month lookup with leap year handling*
- [x] `[impl]` Implement `WinduYear::from_aj()` (done); full `JavaneseYear` from AJ number stubbed
- [x] `[impl]` Implement Pranata Masa from JDN (solar position relative to ~June 21 solstice/Kasa)
      *Formula: `(jdn - KASA_REF).rem_euclid(365)` → lookup in `PRANATA_MASA_SOLAR_OFFSETS`*
- [x] `[impl]` Implement `KurupRecord` with `KURUP_ASAPON_START_JDN` and `KURUP_ASAPON_END_JDN`
      annotated with source-tier doc comments (Danudji 2006 primary, cross-validated)
- [ ] `[impl]` Stub supra-windu group names with citation (Danudji 2006)
- [ ] `[doc]` Add `jawa` entry to workspace `SOURCES.md` with verification-tier table:
      Karjanto & Beauducel (2020) ✅ primary + digitally verifiable;
      Dershowitz & Reingold (4th ed.) ✅ primary + widely cited;
      Danudji (2006) ⚠️ primary, print-only, triangulation required;
      Wikipedia + `beaudu/weton` ℹ️ cross-check only
- [x] `[impl]` Implement `CalendarDate` for `JavaneseDay` (stub — returns NotImplemented for unconverted dates)
- [x] `[impl]` Implement `CalendarMetadata` for `JavaneseDay` (done)
- [ ] `[impl]` Gate `no_std` correctly within module
- [x] `[test]` Known anchor: JDN 2317690 → 1 Sura 1555 AJ, Jumat Legi, wuku Sinta pos 1
      *Implemented in `known_anchor_epoch` test - all values verified*
- [x] `[test]` Known anchor: 1945-08-17 (Proklamasi) → Jumat Legi
      *Implemented in `known_anchor_proklamasi` - saptawara verified, AJ year ~1877*
- [x] `[test]` Kurup boundary: 1936-03-24 → Selasa Pon
      *Implemented in `kurup_boundary_1936` - Selasa (1) ✓, Pon (2) ✓, Wetonan ✓*
      *Note: Windu year at boundary needs cross-validation against Danudji (2006)*
- [x] `[test]` Windu year: `WinduYear::from_aj(1959) == WinduYear::Wawu`
- [ ] `[test]` Round-trip: 500 random JDNs within AJ 1555–2474
- [ ] `[test]` `--no-default-features` + WASM build check

---

### `hijriyah` module

> Detailed execution plan tracked at `~/.windsurf/plans/hijriyah-implementation-a99a87.md`.

- [ ] `[arch]` Confirm GPL-3.0-only exclusion (misykat) and record Option A decision in `src/hijriyah/DECISION.md`
- [ ] `[arch]` Define module structure (`src/hijriyah/mod.rs`, `arithmetic.rs`, `types.rs`, `holidays.rs`, `metadata.rs`)
- [ ] `[arch]` Specify arithmetic sources (Dershowitz-Reingold Ch. 6, Meeus Ch. 9) and cite in rustdoc
- [ ] `[arch]` Declare `tabular_date()` vs `indonesian_government_date()` behavior and supported Hijri range (≥1–1600 AH)
- [ ] `[impl]` Create module skeleton in `src/hijriyah/`
- [ ] `[impl]` Implement `hijri_to_jdn` and `jdn_to_hijri` per D-R Eq. 6.2–6.3 (tabular, Thursday epoch, JDN 1948439 start)
- [ ] `[impl]` Implement leap-year logic (years 2,5,7,10,13,16,18,21,24,26,29 in each 30-year cycle) and expose `HijriDay::is_leap_year`
- [ ] `[impl]` Build `HijriDay` struct with month metadata (Arabic + Indonesian names), `day_of_year`, `pasaran` field
- [ ] `[impl]` Implement Pasaran calculation `(jdn + 2) % 5` as standalone helper (no dependency on `jawa`)
- [ ] `[impl]` Implement holiday helpers: `maulid_jdn`, `isra_miraj_jdn`, `idul_fitri_jdn`, `idul_adha_jdn`, `haul_jdn`
- [ ] `[impl]` Provide `tabular_date()` and stub `indonesian_government_date()` with `stub!()` message referencing Kemenag data need
- [ ] `[impl]` Document algorithm choice; write exclusion rationale in `DECISION.md`; cite references in `SOURCES.md`
- [ ] `[impl]` Implement `CalendarDate` + `CalendarMetadata` traits for `HijriDay`
- [ ] `[test]` Anchor JDNs: 1 Muharram 1 AH (1948439), 1043 AH (2317690), 1355 AH (2428252), 1446 AH (2460494)
- [ ] `[test]` Holiday equality: ensure `idul_fitri_jdn(y) == hijri_to_jdn(y, 10, 1)` etc.
- [ ] `[test]` Pasaran check: `HijriDay::from_jdn(2317690)?.pasaran == Pasaran::Legi` (Jumat Legi)
- [ ] `[test]` Round-trip property: 1000 random JDNs within 1–1600 AH
- [ ] `[test]` no_std + WASM builds: `cargo build/test -p nusantara-calendar --no-default-features --features hijriyah`

---

### `chinese_nusantara` module

- [ ] `[arch]` Audit `nongli` v0.4.1 API: `ChineseDate`, `SolarTerm`; confirm no JDN interface
- [ ] `[arch]` Design bridge: `gregorian_to_jdn` → `nongli::ChineseDate::from_gregorian(NaiveDate)`
      Note: bridge via `jdn_to_gregorian` → `NaiveDate::from_ymd_opt`
- [ ] `[arch]` Document that this module is `std`-only in `Cargo.toml` and rustdoc
- [ ] `[impl]` Create module skeleton in `src/chinese_nusantara/`
- [ ] `[impl]` Add `nongli = "0.4.1"` to `nusantara-calendar` Cargo.toml (already present)
- [ ] `[impl]` Implement `ChineseNusantaraDay` wrapping `nongli::ChineseDate`
- [ ] `[impl]` Implement `from_jdn` bridge via `jdn_to_gregorian` → `chrono::NaiveDate`
- [ ] `[impl]` Add `Shio` (zodiac) enum with 12 variants + Indonesian names
- [ ] `[impl]` Implement `Shio::from_chinese_year(year: i64) -> Shio`
- [ ] `[impl]` Implement `cap_go_meh_jdn(chinese_year: i64) -> i64` (15th day, 1st month)
- [ ] `[impl]` Implement `imlek_jdn(chinese_year: i64) -> i64` (1st day, 1st month)
- [ ] `[impl]` Implement Weton Tionghoa (Pasaran intersection; same formula as hijriyah)
- [ ] `[impl]` Document Singkawang vs Solo/Semarang Cap Go Meh variant in rustdoc
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Imlek 2026: verify Chinese New Year 1st day maps to 2026-01-29 Gregorian
- [ ] `[test]` Cap Go Meh 2026: verify 15th day 1st month maps to 2026-02-12 Gregorian
- [ ] `[test]` Shio: verify 2026 is year of the Horse (Kuda)
- [ ] `[test]` Round-trip: 200 dates (nongli range: 1900–2100)

### Release
- [ ] `[rel]` Integration test: balinese + jawa + hijriyah + chinese_nusantara all compile together
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.2.0]` section
- [ ] `[rel]` Dry-run: `cargo publish --dry-run -p nusantara-calendar --features all-calendars`
- [ ] `[rel]` Tag and publish `nusantara-calendar-v0.2.0`

---

## v0.3.0 — `batak` module

> **Scope:** Batak Porhalaan with Toba, Karo, Simalungun variants.
> **Tag:** `nusantara-calendar-v0.3.0`

### `batak` module

- [ ] `[arch]` Read and summarize: "A Lunar-Star Calendar", preprints.org/manuscript/202404.0235 (2024)
- [ ] `[arch]` Map Toba Porhalaan 12-month system to data types; identify intercalation rule
- [ ] `[arch]` Specify `tabular` behavior (default, no_std) vs `astronomical` feature (runtime calc)
- [ ] `[arch]` Document sub-group variant divergences: Karo, Simalungun vs Toba core
- [ ] `[impl]` Implement `src/batak/mod.rs` within `nusantara-calendar`
- [ ] `[impl]` Implement `const` Toba month names array (12 + 1 intercalary: Ihuthon)
- [ ] `[impl]` Implement `const` Hara day-name cycle (30 entries)
- [ ] `[impl]` Implement tabular Porhalaan: JDN → lunar month via Islamic-style 12/13-month cycle
- [ ] `[impl]` Implement `BatakDay::from_jdn_tabular()`
- [ ] `[impl]` Implement `BatakDay::from_jdn_astronomical(jdn, lat, lon)` (feature-gated `astronomical`)
      using `libm` for heliacal angle computation at Lake Toba (~2.6°N)
- [ ] `[impl]` Stub Karo variant month names with citation
- [ ] `[impl]` Stub Simalungun variant month names with citation
- [ ] `[impl]` Stub Datu auspiciousness system with citation (Schreiner 1972)
- [ ] `[impl]` Implement `CalendarDate` (calls `from_jdn_tabular`), `CalendarMetadata`
- [ ] `[test]` Tabular: verify known Toba festival dates against preprint data
- [ ] `[test]` Round-trip: 200 JDNs, tabular mode
- [ ] `[test]` Astronomical: verify Orion first-rise JDN at lat=2.6°N for 2025 matches
      published astronomical data (use USNO or JPL Horizons reference)
- [ ] `[test]` `--no-default-features` + `--features batak,astronomical` builds both pass

### Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.3.0]`
- [ ] `[rel]` Tag and publish `nusantara-calendar-v0.3.0`

---

## v0.4.0 — `sunda` + `tengger` modules

> **Tag:** `nusantara-calendar-v0.4.0`

### `sunda` module

- [ ] `[arch]` Research Kala Sunda Saka epoch and relationship to Balinese Saka
- [ ] `[arch]` Document Pranatamangsa Sunda solar epoch difference vs Javanese
- [ ] `[impl]` Implement `src/sunda/mod.rs` — Kala Sunda (Saka-derived arithmetic, solar year)
- [ ] `[impl]` Implement Pranatamangsa Sunda (12 seasons, Sundanese month names)
- [ ] `[impl]` Implement Naga Tahun cycle (8-direction year taboo)
- [ ] `[impl]` Stub Sunda Wiwitan (Baduy oral tradition) with citation: Garna (1993)
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Pranatamangsa Sunda: verify season 1 (Kasa) start date aligns with cited source
- [ ] `[test]` Round-trip: 200 JDNs

### `tengger` module

- [ ] `[arch]` Read Hefner (1985) for Kasada computation rules; identify Saka base
- [ ] `[arch]` Confirm Unan-unan 5-year cycle is distinct from Javanese Windu
- [ ] `[impl]` Implement `src/tengger/mod.rs` — Tengger Saka calendar (Hindu-derived)
- [ ] `[impl]` Implement `kasada_jdn(tengger_year: u32) -> i64`
      (14th day, 12th month Kasada = annual pilgrimage to Bromo)
- [ ] `[impl]` Implement Unan-unan 5-year purification cycle
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Kasada 2026: verify JDN maps to the correct Gregorian date
      (cross-reference with Bromo ceremony news sources)
- [ ] `[test]` Round-trip: 200 JDNs

### Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.4.0]`
- [ ] `[rel]` Tag and publish `nusantara-calendar-v0.4.0`

---

## v0.5.0 — `bugis` + `sasak` + `dayak` modules

> **Scope:** Observation-dependent modules; partial stubs accepted.
> **Tag:** `nusantara-calendar-v0.5.0`

### `bugis` module

- [ ] `[arch]` Read Pelras, *The Bugis* (1996) — document calendar sections
- [ ] `[impl]` Implement `src/bugis/mod.rs` — 12-month Bugis lunar calendar (Islamic-influenced)
- [ ] `[impl]` Implement `const` pre-Islamic Bugis month names (12 entries)
- [ ] `[impl]` Implement Siri' timing (lunar phase lookup)
- [ ] `[impl]` Implement `tudang_sipulung_jdn(year)` stub: Pleiades first rise,
      cite astronomical source; provide tabular approximation for ~3°S latitude
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Known anchor: verify Idul Fitri 1446 AH Bugis date alignment with Hijriyah
- [ ] `[test]` `--no-default-features` build

### `sasak` module

- [ ] `[arch]` Read Taufiq et al. — document Rowot new year and Bau Nyale timing
- [ ] `[impl]` Implement `src/sasak/mod.rs` — 12-month Sasak lunar calendar with Sasak-language month names
- [ ] `[impl]` Implement `rowot_new_year_jdn(year)`:
      Pleiades first rise above eastern horizon, Lombok (~8.6°S); tabular fallback
- [ ] `[impl]` Implement `bau_nyale_jdn(year)`: 10th month, specific lunar day lookup
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Bau Nyale 2025/2026: verify Gregorian date against published festival announcements
- [ ] `[test]` Round-trip: 150 JDNs

### `dayak` module

- [ ] `[arch]` Read Schärer (1963) *Ngaju Religion* — document Kaharingan calendar phases
- [ ] `[impl]` Implement `src/dayak/mod.rs` — 12-month Kaharingan agricultural cycle (Pleiades-keyed)
- [ ] `[impl]` Implement `const` Ngaju month names (12 entries); stub Kayan/Kenyah variants
- [ ] `[impl]` Implement Tewah ritual timing (lunar phase + agricultural cycle intersection)
- [ ] `[impl]` Stub Iban, Kenyah, Murut variant month names with citations
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Month boundary: verify Dayak new year approximate JDN vs Pleiades season

### Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.5.0]`
- [ ] `[rel]` Tag and publish `nusantara-calendar-v0.5.0`

---

## v0.6.0 — `toraja` + `minangkabau` modules

> **Tag:** `nusantara-calendar-v0.6.0`

### `toraja` module

- [ ] `[arch]` Read Nooy-Palm (1979) *The Sa'dan Toraja* Vol. 1 — extract month names and
      Rambu Solo' / Rambu Tuka' timing rules
- [ ] `[impl]` Implement `src/toraja/mod.rs` — 12-month Toraja lunar calendar with Toraja-language month names
- [ ] `[impl]` Implement `rambu_solo_season_jdn(year)` (post-harvest period, approx. June–Oct)
- [ ] `[impl]` Implement `manene_jdn(year)` (August cycle; keyed to post-harvest lunar phase)
- [ ] `[impl]` Implement Rambu Tuka' season (planting/prosperity; inverse of Rambu Solo')
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Ma'nene' 2024/2025: verify approximate date against documented ceremony records

### `minangkabau` module

- [ ] `[arch]` Read Kato (1982) *Matriliny and Migration* — document adat calendar obligations
- [ ] `[impl]` Implement `src/minangkabau/mod.rs` — Hijriyah-overlay base (reuse `jdn_to_hijri` arithmetic locally; do not import `hijriyah` module)
- [ ] `[impl]` Implement `turun_ka_sawah_jdn(year)`:
      Pleiades visibility + lunar month conjunction, tabular fallback for 0° latitude
- [ ] `[impl]` Implement `hari_raya_adat_jdn(year)` stub with citation
- [ ] `[impl]` Implement matrilineal clan cycle time-obligation lookup (stub, adat-dependent)
- [ ] `[impl]` Implement `CalendarDate`, `CalendarMetadata`
- [ ] `[test]` Verify Turun ka Sawah approximate season against Padang agricultural calendar

### Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.6.0]`
- [ ] `[rel]` Tag and publish `nusantara-calendar-v0.6.0`

---

## v0.7.0 — `dewasa_engine` module

> **Scope:** Cross-calendar auspiciousness correlator. Depends on all prior modules.
> **Tag:** `nusantara-calendar-v0.7.0`

### `dewasa_engine` module

- [ ] `[arch]` Finalize `NusantaraDay` struct field list; confirm `Option<_>` for all ethnic modules
- [ ] `[arch]` Define `CrossCalendarVerdict` consensus algorithm (most cautious non-Unknown wins)
- [ ] `[arch]` Define `CalendarConflict` struct: which modules, what levels, what dates
- [ ] `[arch]` Decide feature flags for ethnic modules in `dewasa_engine`:
      `features = ["batak", "sunda", "tengger", "bugis", "sasak", "dayak", "toraja", "minangkabau"]`
      Default: only the 4 core modules always included
- [ ] `[impl]` Implement `src/dewasa_engine/mod.rs` — `std`-only declaration
- [ ] `[impl]` Implement `NusantaraDay::from_gregorian(y, m, d)` calling all enabled modules
- [ ] `[impl]` Implement `NusantaraDay::from_jdn(jdn: i64)` as primary constructor
- [ ] `[impl]` Implement `CrossCalendarVerdict::compute(verdicts: &[(name, AuspiciousnessLevel)])`
- [ ] `[impl]` Implement conflict detection: opposite sides of Neutral across 2+ calendars
- [ ] `[impl]` Implement `overall` consensus: most cautious non-Unknown level
- [ ] `[impl]` Add `serde` feature for full `NusantaraDay` serialization (JSON-ready for API)
- [ ] `[test]` Smoke test: `NusantaraDay::from_gregorian(2026, 3, 21)` returns valid struct
- [ ] `[test]` Conflict detection: construct a day with known opposing auspiciousness values
- [ ] `[test]` Serialization: `serde_json::to_string(&day)` round-trips correctly
- [ ] `[test]` Feature-gated ethnic modules: build with/without `batak` feature

### Release
- [ ] `[rel]` Update `CHANGELOG.md` — `## [0.7.0]`
- [ ] `[rel]` Tag and publish `nusantara-calendar-v0.7.0`

---

## v1.0.0 — API Stabilization

> **Scope:** Freeze public API. Resolve all stubs where algorithm is now available.
> Complete documentation. Production-ready for `dedauh.id`.
> **Tags:** `calendar-core-v1.0.0`, `nusantara-calendar-v1.0.0`

### Pre-release Audit
- [ ] `[arch]` Review all `stub!()` calls across all modules
      — for each: either resolve with a verified algorithm or confirm it remains a stub
- [ ] `[arch]` Confirm `#[non_exhaustive]` on all enums that may gain variants post-1.0
- [ ] `[arch]` Audit all `CalendarMetadata::description()` / `cultural_origin()` — no empty strings
- [ ] `[arch]` Resolve `indonesian_government_date()` stub in `hijriyah`
      (requires Kemenag hisab algorithm source; stub if still unresolvable)
- [ ] `[arch]` Full license audit via `cargo deny check` — confirm no GPL transitive deps

### API Hardening
- [ ] `[impl]` Add `#[must_use]` to all `from_jdn` / `from_gregorian` return values
- [ ] `[impl]` Add `#[inline]` to hot-path cycle arithmetic functions
- [ ] `[impl]` Ensure all public types implement `Debug`, `Clone`, `PartialEq`, `Eq`
- [ ] `[impl]` Add `Hash` to all enums (required for use as `HashMap` keys)
- [ ] `[impl]` Verify `serde` round-trips for all public structs across all modules
- [ ] `[impl]` Verify `wasm-bindgen` exports compile and tree-shake correctly

### Documentation
- [ ] `[doc]` Complete rustdoc coverage: `cargo doc --no-deps` with zero warnings
- [ ] `[doc]` Add `# Examples` section to every public `fn`
- [ ] `[doc]` Add `# Panics` / `# Errors` sections where applicable
- [ ] `[doc]` Write module-level `//!` narrative for all 13 calendar modules
- [ ] `[doc]` Finalize `README.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`
- [ ] `[doc]` Write `SOURCES.md` at workspace root — master list of all cited references
- [ ] `[doc]` Write `STUBS.md` at workspace root — inventory of all `stub!()` calls with
      current status (pending / in-progress / will-not-implement + reason)

### Testing
- [ ] `[test]` Achieve ≥80% line coverage across all non-stub code
- [ ] `[test]` Add property-based tests (via `proptest`) for all `CalendarDate::from_jdn` impls
- [ ] `[test]` Add benchmark suite (`criterion`) for JDN pivot functions
- [ ] `[test]` Full WASM build matrix: all `no_std` modules on `wasm32-unknown-unknown`
- [ ] `[test]` MSRV check: `cargo +1.80 test --workspace`

### CI Additions for v1.0
- [ ] `[ci]` Add `cargo semver-checks` to CI (detects unintentional breaking changes)
- [ ] `[ci]` Add coverage report (via `cargo llvm-cov`) with ≥80% gate
- [ ] `[ci]` Publish docs to `docs.rs` and verify all intra-doc links resolve

### Release
- [ ] `[rel]` Write `CHANGELOG.md` — `## [1.0.0]` with migration notes from 0.x
- [ ] `[rel]` Bump both crate versions to `1.0.0`
- [ ] `[rel]` `cargo publish` in dependency order: `calendar-core` then `nusantara-calendar`
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
      across modules without coupling

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
