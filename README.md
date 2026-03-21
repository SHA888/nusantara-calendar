# nusantara-calendar

[![CI](https://img.shields.io/github/actions/workflow/status/your-org/nusantara-calendar/ci.yml?branch=main&label=CI)](https://github.com/your-org/nusantara-calendar/actions)
[![crates.io](https://img.shields.io/crates/v/calendar-core.svg)](https://crates.io/crates/calendar-core)
[![docs.rs](https://docs.rs/calendar-core/badge.svg)](https://docs.rs/calendar-core)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV: 1.75](https://img.shields.io/badge/MSRV-1.75-orange.svg)](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)

A Rust workspace covering every traditional calendar system with a documented algorithmic
basis across the Indonesian archipelago — from Sabang (Aceh) to Merauke (Papua) — plus the
three supra-ethnic calendars (Hijriyah, Javanese, Chinese) present on standard Indonesian
almanacs.

**This is the only library in any language targeting all of these calendar systems in a
single, algorithmically grounded, source-attributed codebase.**

---

## Crates

| Crate | Description | `no_std` | Status |
|---|---|---|---|
| [`calendar-core`] | Shared traits, JDN pivot, error types, `stub!()` macro | ✅ | v0.1 |
| [`balinese-calendar`] | Balinese Saka-Wuku calendar (Pawukon, Wewaran, Sasih) | ✅ | v0.2 |
| [`jawa`] | Javanese Wetonan, Pawukon, Windu, Pranata Masa | ✅ | v0.2 |
| [`hijriyah`] | Islamic lunar calendar + Indonesian Kemenag extensions | ✅ | v0.2 |
| [`chinese-nusantara`] | Chinese lunisolar calendar, Peranakan context | ❌ (std) | v0.2 |
| [`batak`] | Batak Porhalaan (Toba, Karo, Simalungun variants) | ✅ / ❌† | v0.3 |
| [`sunda`] | Sundanese Kala Sunda, Pranatamangsa Sunda | ✅ | v0.4 |
| [`tengger`] | Tengger (Bromo) Hindu calendar, Kasada ceremony | ✅ | v0.4 |
| [`bugis`] | Bugis-Makassar lunar calendar, Tudang Sipulung | ✅ | v0.5 |
| [`sasak`] | Sasak Rowot calendar, Bau Nyale festival | ✅ | v0.5 |
| [`dayak`] | Dayak Kaharingan agricultural calendar | ✅ | v0.5 |
| [`toraja`] | Toraja ritual calendar (Rambu Solo', Rambu Tuka') | ✅ | v0.6 |
| [`minangkabau`] | Minangkabau agricultural + Islamic overlay | ✅ | v0.6 |
| [`dewasa-engine`] | Cross-calendar auspiciousness correlator | ❌ (std) | v0.7 / v1.0 |

† `batak`: `no_std` for tabular feature; `std` required for `astronomical` feature (heliacal observation math).

---

## Design Principles

**Julian Day Number (JDN) as the universal pivot.** Every calendar converts through JDN.
This means any two calendar systems can interoperate without knowing about each other directly.

**Source attribution is a first-class requirement.** Every public type carries `# Sources`
documentation linking to citable academic or primary sources. Where an algorithm is unknown,
the crate exposes a `stub!()` returning `CalendarError::NotImplemented` — it never silently
fabricates data.

**Algorithmic correctness over feature completeness.** A `stub!()` with a citation is
strictly better than a plausible-but-unverified implementation.

---

## Quick Start

Add the workspace crates you need to your `Cargo.toml`:

```toml
[dependencies]
calendar-core      = "0.1"
balinese-calendar  = "0.2"
jawa               = "0.2"
hijriyah           = "0.2"
```

### Convert a Gregorian date to Balinese

```rust
use balinese_calendar::BalineseDay;
use calendar_core::CalendarDate;

let day = BalineseDay::from_gregorian(2026, 3, 21)?;
println!("Wuku:    {}", day.wuku);
println!("Saptawara: {}", day.saptawara);
println!("Pancawara: {}", day.pancawara);
println!("Sasih:   {}", day.sasih);
```

### Convert a Gregorian date to Javanese

```rust
use jawa::JavaneseDay;
use calendar_core::CalendarDate;

let day = JavaneseDay::from_gregorian(2026, 3, 21)?;
println!("Weton:     {} {}", day.saptawara, day.pasaran);
println!("Wuku:      {}", day.wuku);
println!("Windu year: {:?}", day.windu_year);   // WinduYear::Wawu (AJ 1959)
println!("Kurup:     {}", day.kurup);            // Alip Selasa Pon
```

### Get a unified cross-calendar day (dewasa-engine)

```rust
use dewasa_engine::{NusantaraDay, Activity};

let day = NusantaraDay::from_gregorian(2026, 3, 21)?;
let verdict = &day.cross_auspiciousness[&Activity::Marriage];
println!("Overall:  {:?}", verdict.overall);
println!("Conflicts: {:?}", verdict.conflicts);
```

---

## Calendrical Scope

### Fully algorithmic (no observation required)
- Balinese Saka-Wuku
- Javanese (Wetonan, Pawukon, Windu, Pranata Masa)
- Hijriyah tabular
- Chinese lunisolar (via `nongli`)
- Tengger (Saka-based)
- Sundanese (Saka-derived, pre-Sultan Agung)

### Observation-dependent (tabular fallback provided)
- Batak Porhalaan — Orion/Scorpius heliacal rising at Lake Toba (~2.6°N)
- Sasak Rowot — Pleiades first rising above eastern horizon
- Bugis Tudang Sipulung — Pleiades visibility
- Minangkabau Turun ka sawah — Pleiades + lunar conjunction
- Hijriyah `indonesian_government_date()` — rukyat/hisab combination

### Documented stubs (oral tradition / unpublished algorithm)
- Baduy (Sunda Wiwitan) ceremonial calendar
- Batak supra-windu group names (Sengara, Langkir, etc.)
- Karo/Simalungun/Pakpak variant divergences beyond month names

---

## Key Dates & Constants

```rust
// jawa crate
pub const SULTAN_AGUNG_EPOCH_JDN: i64 = 2317690; // 1633-07-08 Gregorian

// Current kurup: Alip Selasa Pon, 1936-03-24 → 2052-08-25
// Current AJ year (March 2026): ~1959, Windu year = Wawu
```

---

## `no_std` and WASM

Core computation crates (`calendar-core`, `jawa`, `balinese-calendar`, `hijriyah` Option A,
and most ethnic crates) are `no_std + alloc` compatible. They compile to WASM32:

```sh
cargo build --target wasm32-unknown-unknown -p calendar-core
cargo build --target wasm32-unknown-unknown -p jawa
```

`chinese-nusantara` and `dewasa-engine` are `std`-only and should be feature-gated in WASM
contexts.

---

## Feature Flags

| Flag | Applies to | Effect |
|---|---|---|
| `serde` | all crates | Derives `Serialize`/`Deserialize` on all public types |
| `wasm` | all crates | Derives `wasm_bindgen` exports |
| `astronomical` | `batak`, `sasak`, `bugis`, `minangkabau` | Enables observation-based computation (requires `std`) |

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

> **Note on `chinese-nusantara`**: depends on `nongli` (MIT). License compatible.
>
> **Note on `hijriyah`**: does **not** depend on `misykat` (GPL-3.0). Hijri date
> arithmetic is reimplemented independently from Dershowitz-Reingold Ch. 6 and Meeus Ch. 9,
> keeping this crate MIT/Apache-2.0. See `hijriyah/SOURCES.md` for full attribution.

---

## Contributing

Contributions are welcome, subject to one strict rule: **every algorithmic claim requires
a citable source.** Undocumented "it works on my almanac" patches will not be merged.

See [CONTRIBUTING.md](CONTRIBUTING.md) and [ARCHITECTURE.md](ARCHITECTURE.md).

---

## Acknowledgements

- Beauducel & Karjanto (2020), *An ethnoarithmetic excursion into the Javanese calendar*,
  arXiv:2012.10064 — Wetonan congruence formula
- Dershowitz & Reingold, *Calendrical Calculations* (4th ed.) — JDN pivot and Pawukon algorithm
- H. Danudji, *Penanggalan Jawa 120 Tahun Kurup Asapon*, Dahara Prize, 2006 — Kurup table
- "A Lunar-Star Calendar: Inquiry to the Traditional Batak Calendar",
  preprints.org/manuscript/202404.0235 (2024) — Batak Porhalaan
- `balinese-calendar` crate (v0.1.1) — first native Rust Balinese Saka calendar implementation

---

*This library is a temporal computation tool. It is not an astrology app, prayer-time
calculator, timezone converter, or Gregorian calendar utility.*
