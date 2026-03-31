# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.0] - 2026-03-31
### Added
- Initial release of `calendar-core` crate with foundational types and traits
- **Core Types**:
  - `JDN` - Julian Day Number type alias for date conversions
  - `CycleYear` - Type for cycle-year fields (u32)
  - `SubYearPosition` - Type for sub-year positions (u8)
- **Core Traits**:
  - `CalendarDate` - Core interface for calendar implementations with JDN pivot pattern
  - `CalendarMetadata` - Access to calendar metadata and cultural context
  - `HasAuspiciousness` - Optional trait for auspiciousness calculations
- **Error Handling**:
  - `CalendarError` - Comprehensive error types with detailed context
  - `stub!()` macro for marking unimplemented features
- **Date Conversion**:
  - Gregorian ↔ JDN conversion using Fliegel & van Flandern algorithm
  - Reference implementation from Meeus, "Astronomical Algorithms"
- **Features**:
  - `std` - Standard library support (default)
  - `serde` - Serialization support
  - `wasm` - WebAssembly compilation support
  - `no_std` compatibility for embedded systems
- **Documentation**:
  - Complete rustdoc coverage on all public items
  - Crate-level documentation with JDN pivot explanation
  - Intra-doc links and examples
- **Quality Assurance**:
  - 10 unit tests with 100% pass rate
  - 4 documentation tests
  - Clippy linting with pedantic rules
  - MSRV 1.80 support verified

## [Unreleased]
### Added
- Initial documentation scaffolding (`README.md`, `ARCHITECTURE.md`, `SPEC.md`, `TODO.md`).
- Apache-2.0 `LICENSE` and CONTRIBUTING guidelines.
- Hijriyah crate execution plan references replicated across docs.
