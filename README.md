# nusantara-calendar

[![CI](https://img.shields.io/github/actions/workflow/status/SHA888/nusantara-calendar/ci.yml?branch=main&label=CI)](https://github.com/SHA888/nusantara-calendar/actions)
[![crates.io](https://img.shields.io/crates/v/nusantara-calendar.svg)](https://crates.io/crates/nusantara-calendar)
[![docs.rs](https://docs.rs/nusantara-calendar/badge.svg)](https://docs.rs/nusantara-calendar)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV: 1.94](https://img.shields.io/badge/MSRV-1.94-orange.svg)](https://blog.rust-lang.org/2024/11/28/Rust-1.94.0.html)

A comprehensive Rust crate covering every traditional calendar system with a documented algorithmic basis across the Indonesian archipelago. This crate provides unified `calendar-core` trait integration for multiple Indonesian/Nusantara calendar systems.

**This is the only library in any language targeting all of these calendar systems in a single, algorithmically grounded, source-attributed codebase.**

---

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

---

## Documentation

### Workspace Documentation
- **[Main README](README.md)** - This file - workspace overview and quick start
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Design decisions and dependency graph
- **[SPEC.md](SPEC.md)** - Full workspace specification
- **[TODO.md](TODO.md)** - Implementation status and roadmap
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guidelines

### Crate Documentation
- **[calendar-core](crates/calendar-core/README.md)** - Core traits and utilities
- **[calendar-core CHANGELOG](crates/calendar-core/CHANGELOG.md)** - Core library changes
- **[nusantara-calendar](crates/nusantara-calendar/README.md)** - Calendar implementations
- **[nusantara-calendar CHANGELOG](crates/nusantara-calendar/CHANGELOG.md)** - Main library changes

### API Documentation
- **[calendar-core docs.rs](https://docs.rs/calendar-core)** - Core API documentation
- **[nusantara-calendar docs.rs](https://docs.rs/nusantara-calendar)** - Main API documentation

---

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
```

---

## Architecture

### Workspace Structure

`nusantara-calendar` is a workspace containing two crates:

```
nusantara-calendar (workspace)
    calendar-core (separate crate, published to crates.io v0.1.0)
    nusantara-calendar (main crate with feature-gated calendar modules, not yet published)
```

### External Dependencies

The crate leverages official, maintained implementations where available:

- **`balinese-calendar` v0.2** - Official Balinese calendar implementation (separate repo)
- **`nongli` v0.4** - Chinese calendar implementation (`chinese-nusantara` feature)

### calendar-core Integration

All calendar modules implement the `calendar-core` traits:

- `CalendarDate` - Date conversion and validation
- `CalendarMetadata` - Calendar metadata and epoch information
- `HasAuspiciousness` - Auspiciousness calculations (where algorithmically documented)

## Publishing

Both crates can be published independently to crates.io:

```sh
# Publish calendar-core first (already published at v0.1.0)
cargo publish -p calendar-core

# Then publish nusantara-calendar
cargo publish -p nusantara-calendar
```

---

## Design Principles

**Julian Day Number (JDN) as the universal pivot.** Every calendar converts through JDN. This means any two calendar systems can interoperate without knowing about each other directly.

**Leverage official implementations.** Where high-quality, maintained implementations exist (like `balinese-calendar`), this crate wraps them and provides trait integration rather than reimplementing.

**Source attribution is a first-class requirement.** Every public type carries `# Sources` documentation linking to citable academic or primary sources.

**Algorithmic correctness over feature completeness.** A `stub!()` with a citation is strictly better than a plausible-but-unverified implementation.

---

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

---

## `no_std` and WASM

Core computation modules are `no_std + alloc` compatible. They compile to WASM32:

```sh
cargo build --target wasm32-unknown-unknown -p nusantara-calendar \
  --no-default-features --features balinese,jawa,hijriyah
```

`chinese-nusantara` and `dewasa-engine` are `std`-only and should be excluded in WASM contexts.

---

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

---

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

> **Note on dependencies**: This crate depends on `balinese-calendar` (MIT) and may depend on `nongli` (MIT) when the `chinese-nusantara` feature is enabled. All dependencies are license-compatible.

---

## Contributing

Contributions are welcome, subject to one strict rule: **every algorithmic claim requires a citable source.** Undocumented "it works on my almanac" patches will not be merged.

See [CONTRIBUTING.md](CONTRIBUTING.md) and [TODO.md](TODO.md) for implementation status.

---

## Acknowledgements

- Karjanto & Beauducel (2020), *An ethnoarithmetic excursion into the Javanese calendar*, arXiv:2012.10064
- Dershowitz & Reingold, *Calendrical Calculations* (4th ed.) - JDN pivot and Pawukon algorithm
- [`balinese-calendar`](https://github.com/SHA888/balinese-calendar) crate - Official Balinese Saka calendar implementation
- Various Indonesian cultural authorities and academic sources for each calendar system

---

*This library is a temporal computation tool. It is not an astrology app, prayer-time calculator, timezone converter, or Gregorian calendar utility.*
