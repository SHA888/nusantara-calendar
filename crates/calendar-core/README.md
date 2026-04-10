# calendar-core

![crates.io](https://img.shields.io/crates/v/calendar-core.svg)
![docs.rs](https://docs.rs/calendar-core/badge.svg)
![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)
![MSRV: 1.94](https://img.shields.io/badge/MSRV-1.94-orange.svg)

Core traits and utilities for nusantara-calendar workspace, providing foundational types and interfaces for Indonesian calendar systems.

## Overview

`calendar-core` provides the essential building blocks for implementing traditional Indonesian calendar systems in Rust. It defines the core traits, types, and utilities that enable interoperability between different calendar systems while maintaining cultural authenticity and computational accuracy.

## Features

- **Julian Day Number (JDN) System** - Universal pivot for calendar conversions
- **Core Traits** - `CalendarDate`, `CalendarMetadata`, `HasAuspiciousness`
- **Error Handling** - Comprehensive `CalendarError` types
- **Cultural Support** - Indonesian-specific activity types and auspiciousness levels
- **Platform Support** - `no_std`, `std`, and WASM compatibility
- **Gregorian Conversions** - Built-in Gregorian calendar utilities

## Core Components

### Traits

- **`CalendarDate`** - Core interface for date conversions and validation
- **`CalendarMetadata`** - Access to calendar information and cultural context
- **`HasAuspiciousness`** - Cultural auspiciousness evaluations for activities

### Types

- **`JDN`** - Julian Day Number (`i64`) for universal date representation
- **`CycleYear`** - Calendar cycle years (`u32`)
- **`SubYearPosition`** - Sub-year positions like months/days (`u8`)

### Cultural Features

- **`Activity`** - Indonesian cultural activities for auspiciousness evaluation
- **`AuspiciousnessLevel`** - 5-level system for favorability assessment

## Usage

### Basic Calendar Implementation

```rust
use calendar_core::{CalendarDate, CalendarError, JDN};

#[derive(Debug, Clone, PartialEq, Eq)]
struct MyCalendarDate {
    year: i32,
    month: u8,
    day: u8,
}

impl CalendarDate for MyCalendarDate {
    fn from_jdn(jdn: JDN) -> Result<Self, CalendarError> {
        // Convert JDN to calendar date
        todo!("Implement JDN to calendar conversion")
    }

    fn to_jdn(&self) -> JDN {
        // Convert calendar date to JDN
        todo!("Implement calendar to JDN conversion")
    }

    fn calendar_name() -> &'static str {
        "My Calendar System"
    }

    fn validate_range(&self) -> Result<(), CalendarError> {
        // Validate date range
        if self.year < 1 || self.year > 9999 {
            return Err(CalendarError::OutOfRange(
                "Year must be between 1 and 9999".to_string()
            ));
        }
        Ok(())
    }
}
```

### Gregorian Conversions

```rust
use calendar_core::{gregorian_to_jdn, jdn_to_gregorian};

// Convert Gregorian to JDN
let jdn = gregorian_to_jdn(2024, 3, 15);

// Convert back to Gregorian
let (year, month, day) = jdn_to_gregorian(jdn);
assert_eq!((year, month, day), (2024, 3, 15));
```

### Auspiciousness System

```rust
use calendar_core::{HasAuspiciousness, Activity, AuspiciousnessLevel};

impl HasAuspiciousness for MyCalendarDate {
    type Activity = Activity;
    type AuspiciousnessLevel = AuspiciousnessLevel;

    fn auspiciousness_for(&self, activity: &Activity) -> AuspiciousnessLevel {
        match activity {
            Activity::Marriage => AuspiciousnessLevel::Auspicious,
            Activity::Building => AuspiciousnessLevel::Neutral,
            _ => AuspiciousnessLevel::Neutral,
        }
    }

    fn is_auspicious_day(&self) -> bool {
        // General auspiciousness assessment
        true
    }
}
```

## Algorithm Source

The Gregorian to JDN conversion algorithm follows the standard formula from:

**Meeus, Jean.** *Astronomical Algorithms*, 2nd Edition.
Willmann-Blohm, 1998. Chapter 7: "Julian Day".

This provides accurate conversions for the full range of historical dates supported by the JDN system (approximately 262,000 BCE to 262,000 CE).

## Platform Support

This crate supports multiple compilation targets:

- **std**: Standard library with full functionality
- **no_std**: Embedded systems with `alloc` support
- **WASM**: WebAssembly targets for browser usage

## Feature Flags

- `std` (default): Enable standard library support
- `serde`: Enable serialization support
- `wasm`: Enable WASM bindings

## Indonesian Calendar Context

This crate is specifically designed to support the rich diversity of Indonesian calendar systems:

- **Javanese calendar** - Saka and Islamic integration
- **Balinese calendar** - Pawukon cycle system
- **Hijri/Islamic calendar** - Lunar calendar system
- **Chinese calendar** - Lunisolar integration
- **Regional calendars** - Various ethnic calendar systems

Each calendar system can implement the core traits while maintaining cultural authenticity and computational accuracy.

## Error Handling

The crate provides comprehensive error types:

```rust
use calendar_core::CalendarError;

match result {
    Err(CalendarError::OutOfRange(msg)) => println!("Out of range: {msg}"),
    Err(CalendarError::InvalidParameters(msg)) => println!("Invalid params: {msg}"),
    Err(CalendarError::NotImplemented(msg)) => println!("Not implemented: {msg}"),
    Err(CalendarError::ArithmeticError(msg)) => println!("Calculation error: {msg}"),
    Ok(_) => println!("Success!"),
}
```

## Performance Considerations

- JDN conversions are the most computationally expensive operations
- Cache results when performing repeated conversions
- Use the provided Gregorian conversion methods for convenience
- Consider lazy evaluation for complex calendar calculations

## Contributing

Contributions are welcome, subject to one strict rule: **every algorithmic claim requires a citable source.** Undocumented "it works on my almanac" patches will not be merged.

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

Licensed under the [Apache License, Version 2.0](../../LICENSE).

## Acknowledgements

- Beauducel & Karjanto (2020), *An ethnoarithmetic excursion into the Javanese calendar*, arXiv:2012.10064
- Dershowitz & Reingold, *Calendrical Calculations* (4th ed.) - JDN pivot and algorithms
- Meeus, Jean. *Astronomical Algorithms* (2nd ed.) - Gregorian conversion formulas
- Various Indonesian cultural authorities and academic sources

---

*This library is a temporal computation tool. It is not an astrology app, prayer-time calculator, timezone converter, or Gregorian calendar utility.*
