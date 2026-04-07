# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Workspace structure with calendar-core as separate member
- Balinese calendar wrapper around official balinese-calendar crate
- Feature flags for modular calendar system compilation
- Comprehensive documentation for each calendar system

### Changed
- Refactored from single-crate to workspace architecture
- Moved calendar-core to separate crate within workspace
- Updated dependency management for workspace structure

### Fixed
- Balinese wrapper trait implementations
- Feature flag compilation issues
- Workspace member configuration

## [0.1.0] - 2024-03-19

### Added
- Initial release as unified Indonesian calendar system crate
- Workspace structure with calendar-core and nusantara-calendar crates
- Balinese calendar wrapper implementation
- Feature flag system for modular compilation
- Comprehensive trait integration with calendar-core
- Gregorian conversion utilities
- Cultural auspiciousness support
- Platform support for embedded systems and WASM

### Calendar Systems

#### Balinese Calendar (Implemented)
- Wrapper around official `balinese-calendar` v0.2.0 crate
- Full trait implementations: CalendarDate, CalendarMetadata, HasAuspiciousness
- Access to official crate types: Sasih, Wuku, Saptawara, Pancawara
- Comprehensive test suite (9 unit tests + 2 doc tests)
- Cultural authenticity maintained through official crate

#### Other Calendar Systems (Planned)
- **Jawa** - Javanese calendar with Wetonan, Pawukon, Windu, Pranata Masa
- **Hijriyah** - Islamic lunar calendar with Indonesian extensions
- **Batak** - Batak Porhalaan with regional variants
- **Sunda** - Sundanese Kala Sunda system
- **Tengger** - Tenggerese calendar with Kasada ceremony support
- **Bugis** - Buginese Tudang Sipulung calendar
- **Sasak** - Sasak Rowot calendar
- **Dayak** - Dayak Kaharingan agricultural calendar
- **Toraja** - Torajan ritual calendar
- **Minangkabau** - Minangkabau agricultural + Islamic overlay
- **Chinese-Nusantara** - Chinese calendar with Indonesian context
- **Dewasa-Engine** - Cross-calendar auspiciousness correlator

### Technical Features

#### Architecture
- **Workspace Structure**: Two crates (calendar-core + nusantara-calendar)
- **Feature Flags**: Modular compilation for each calendar system
- **Trait Integration**: Unified interface across all calendar systems
- **External Dependencies**: Leverage official implementations where available

#### Platform Support
- **std**: Standard library with full functionality
- **no_std**: Embedded systems with `alloc` support
- **WASM**: WebAssembly targets for browser usage
- **Serde**: Optional serialization support

#### Cultural Features
- **Activity Types**: Indonesian cultural activities for auspiciousness
- **Auspiciousness Levels**: 5-level favorability system
- **Calendar Metadata**: Epoch, cycle, and cultural origin information
- **Source Attribution**: Academic and cultural references for all algorithms

### Documentation
- Complete API documentation with examples
- Cultural context for each calendar system
- Architecture and design principles
- Performance considerations
- Contributing guidelines

### Testing
- Comprehensive test suite for implemented calendars
- Round-trip conversion tests
- Feature flag compilation tests
- Documentation tests
- Platform compatibility tests

### Dependencies

#### External Calendar Crates
- `balinese-calendar` v0.2.0 - Official Balinese calendar implementation
- `nongli` v0.4.0 - Chinese calendar (license audit required)

#### Core Dependencies
- `calendar-core` v0.1.0 - Core traits and utilities
- `thiserror` v2.0 - Error handling
- `serde` v1.0 - Serialization (optional)

### Performance
- Optimized JDN conversions
- Efficient trait implementations
- Minimal compilation overhead with feature flags
- WASM-optimized builds

---

## Migration Guide

### From External Dependencies

If you were previously using `balinese-calendar` directly:

```toml
# Before
[dependencies]
balinese-calendar = "0.2"

# After
[dependencies]
nusantara-calendar = { version = "0.1", features = ["balinese"] }
```

### API Changes

The wrapper provides the same functionality with trait integration:

```rust
// Before
use balinese_calendar::BalineseDate;

// After
use nusantara_calendar::balinese::BalineseDate;
use calendar_core::CalendarDate; // Now has trait implementations
```

---

## Release Notes

### Version 0.1.0

This initial release establishes the foundation for a comprehensive Indonesian calendar system:

1. **Unified Architecture**: Workspace structure with shared core traits
2. **Balinese Implementation**: Production-ready wrapper around official crate
3. **Modular Design**: Feature flags for selective compilation
4. **Cultural Authenticity**: Integration with official implementations
5. **Future-Ready**: Framework for additional calendar systems

The Balinese calendar implementation serves as the reference for how other calendar systems should be integrated, providing both authenticity and interoperability.

### Compatibility

- **Minimum Rust Version**: 1.80
- **Platform Support**: Linux, macOS, Windows, WASM, embedded systems
- **Dependency Requirements**: See Cargo.toml for specific versions
- **License**: Apache-2.0 OR MIT

### Known Limitations

- Only Balinese calendar is fully implemented in v0.1.0
- Other calendar systems are stubbed for future implementation
- Chinese calendar dependency requires license audit
- Some observation-dependent calendars use tabular fallbacks

---

## Future Roadmap

### Version 0.2.0 (Planned)
- Javanese calendar implementation
- Hijriyah calendar implementation
- Enhanced Balinese features
- Performance optimizations
- Additional platform support

### Version 0.3.0 (Planned)
- Batak calendar implementation
- Sundanese calendar implementation
- Tengger calendar implementation
- Cross-calendar comparisons
- Extended auspiciousness features

### Version 1.0.0 (Planned)
- All major Indonesian calendar systems
- Complete dewasa-engine implementation
- Full documentation
- Performance benchmarks
- API stability guarantees

---

## Sources and References

### Algorithm Sources
- Beauducel & Karjanto (2020). *An ethnoarithmetic excursion into the Javanese calendar*
- Dershowitz & Reingold. *Calendrical Calculations* (4th ed.)
- Meeus, Jean. *Astronomical Algorithms* (2nd ed.)

### Cultural Sources
- Various Indonesian cultural authorities and academic sources
- Traditional calendar practitioners and experts
- Government calendar standards and publications

### Technical References
- Official `balinese-calendar` crate documentation
- Calendar-core trait specifications
- WASM and embedded systems optimization guides

---

*For detailed documentation, see the [README.md](README.md) file.*
