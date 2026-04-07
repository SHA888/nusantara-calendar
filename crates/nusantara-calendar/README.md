# nusantara-calendar

![crates.io](https://img.shields.io/crates/v/nusantara-calendar.svg)
![docs.rs](https://docs.rs/nusantara-calendar/badge.svg)
![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)
![MSRV: 1.80](https://img.shields.io/badge/MSRV-1.80-orange.svg)

A comprehensive Rust crate covering every traditional calendar system with a documented algorithmic basis across the Indonesian archipelago. This crate provides unified `calendar-core` trait integration for multiple Indonesian/Nusantara calendar systems.

**This is the only library in any language targeting all of these calendar systems in a single, algorithmically grounded, source-attributed codebase.**

## Features

This crate uses feature flags to minimize compilation time and binary size:

- `balinese` - Balinese Saka calendar (wraps official `balinese-calendar` crate)
- `jawa` - Javanese calendar system
- `hijriyah` - Islamic calendar
- `batak` - Batak calendar
- `sunda` - Sundanese calendar
- `tengger` - Tenggerese calendar
- `bugis` - Buginese calendar
- `sasak` - Sasak calendar
- `dayak` - Dayak calendar
- `toraja` - Torajan calendar
- `minangkabau` - Minangkabau calendar
- `chinese-nusantara` - Chinese calendar adapted for Indonesian context
- `dewasa-engine` - Auspiciousness calculation engine

### Convenience Feature Groups
- `all-calendars` - Enable all calendar systems
- `all` - Enable all features including dewasa-engine

## Quick Start

Add `nusantara-calendar` to your `Cargo.toml` with the features you need:

```toml
[dependencies]
nusantara-calendar = { version = "0.1", features = ["balinese", "jawa"] }
```

### Convert a Gregorian date to Balinese

```rust
use nusantara_calendar::balinese::BalineseDate;
use calendar_core::CalendarDate;

let date = BalineseDate::from_ymd(2026, 3, 19)?;
println!("Saka year: {}", date.saka_year);
println!("Wuku:      {}", date.wuku);
println!("Sasih:     {}", date.sasih);
```

### Convert a Gregorian date to Javanese (when implemented)

```rust
use nusantara_calendar::jawa::JavaneseDay;
use calendar_core::CalendarDate;

let day = JavaneseDay::from_gregorian(2026, 3, 21)?;
println!("Weton:     {} {}", day.saptawara, day.pasaran);
println!("Wuku:      {}", day.wuku);
```

### Get a unified cross-calendar day (dewasa-engine)

```rust
use nusantara_calendar::dewasa_engine::{NusantaraDay, Activity};

let day = NusantaraDay::from_gregorian(2026, 3, 21)?;
let verdict = &day.cross_auspiciousness[&Activity::Marriage];
println!("Overall:  {:?}", verdict.overall);
println!("Conflicts: {:?}", verdict.conflicts);
```

## Architecture

### Workspace Structure

`nusantara-calendar` is a workspace containing two crates:

```
nusantara-calendar (workspace)
    calendar-core (separate crate, published to crates.io)
    nusantara-calendar (main crate with calendar systems)
```

### External Dependencies

The crate leverages official, maintained implementations where available:

- **`balinese-calendar` v0.2** - Official Balinese calendar implementation
- **`nongli` v0.4** - Chinese calendar implementation (license audit required)

### calendar-core Integration

All calendar systems implement the `calendar-core` traits:

- `CalendarDate` - Date conversion and validation
- `CalendarMetadata` - Calendar metadata and epoch information  
- `HasAuspiciousness` - Auspiciousness calculations

## Publishing

Both crates can be published independently to crates.io:

```sh
# Publish calendar-core first
cargo publish -p calendar-core

# Then publish nusantara-calendar
cargo publish -p nusantara-calendar
```

## Design Principles

**Julian Day Number (JDN) as the universal pivot.** Every calendar converts through JDN. This means any two calendar systems can interoperate without knowing about each other directly.

**Leverage official implementations.** Where high-quality, maintained implementations exist (like `balinese-calendar`), this crate wraps them and provides trait integration rather than reimplementing.

**Source attribution is a first-class requirement.** Every public type carries `# Sources` documentation linking to citable academic or primary sources.

**Algorithmic correctness over feature completeness.** A `stub!()` with a citation is strictly better than a plausible-but-unverified implementation.

## Calendrical Scope

### Fully algorithmic (no observation required)
- Balinese Saka-Wuku (via official `balinese-calendar` crate)
- Javanese (Wetonan, Pawukon, Windu, Pranata Masa)
- Hijriyah tabular
- Tengger (Saka-based)
- Sundanese (Saka-derived, pre-Sultan Agung)

### Observation-dependent (tabular fallback provided)
- Batak Porhalaan
- Sasak Rowot
- Bugis Tudang Sipulung
- Minangkabau Turun ka sawah
- Hijriyah `indonesian_government_date()`

### Documented stubs (oral tradition / unpublished algorithm)
- Baduy (Sunda Wiwitan) ceremonial calendar
- Batak supra-windu group names
- Karo/Simalungun/Pakpak variant divergences

## `no_std` and WASM

Core computation crates are `no_std + alloc` compatible. They compile to WASM32:

```sh
cargo build --target wasm32-unknown-unknown --features balinese,jawa
```

`chinese-nusantara` and `dewasa-engine` are `std`-only and should be feature-gated in WASM contexts.

## Feature Flags

| Flag | Effect |
|---|---|
| `std` | Enable standard library (default) |
| `serde` | Derives `Serialize`/`Deserialize` on all public types |
| `wasm` | Derives `wasm_bindgen` exports |
| `balinese` | Enable Balinese calendar (depends on `balinese-calendar`) |
| `jawa` | Enable Javanese calendar |
| `hijriyah` | Enable Islamic calendar |
| `all-calendars` | Enable all calendar systems |
| `all` | Enable all features |

## Key Dates & Constants

```rust
// jawa crate
pub const SULTAN_AGUNG_EPOCH_JDN: i64 = 2317690; // 1633-07-08 Gregorian

// Current kurup: Alip Selasa Pon, 1936-03-24 -> 2052-08-25
// Current AJ year (March 2026): ~1959, Windu year = Wawu
```

## License

Licensed under the [Apache License, Version 2.0](../../LICENSE).

> **Note on dependencies**: This crate depends on `balinese-calendar` (MIT) and may depend on `nongli` (MIT) when the `chinese-nusantara` feature is enabled. All dependencies are license-compatible.

## Contributing

Contributions are welcome, subject to one strict rule: **every algorithmic claim requires a citable source.** Undocumented "it works on my almanac" patches will not be merged.

See [CONTRIBUTING.md](../../CONTRIBUTING.md) and [TODO.md](../../TODO.md) for implementation status.

## Acknowledgements

- Beauducel & Karjanto (2020), *An ethnoarithmetic excursion into the Javanese calendar*, arXiv:2012.10064
- Dershowitz & Reingold, *Calendrical Calculations* (4th ed.) - JDN pivot and Pawukon algorithm
- [`balinese-calendar`](https://github.com/SHA888/balinese-calendar) crate - Official Balinese Saka calendar implementation
- Various Indonesian cultural authorities and academic sources for each calendar system

---

*This library is a temporal computation tool. It is not an astrology app, prayer-time calculator, timezone converter, or Gregorian calendar utility.*
