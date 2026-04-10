# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of calendar-core crate
- Core trait definitions for Indonesian calendar systems
- Julian Day Number (JDN) conversion utilities
- Comprehensive error handling system
- Cultural activity and auspiciousness enums
- Platform support for `no_std`, `std`, and WASM
- Gregorian calendar conversion functions
- Documentation and examples

### Changed
- Moved from external dependency to workspace member
- Updated trait implementations to match Indonesian calendar requirements

### Fixed
- JDN conversion algorithms for historical accuracy
- Type definitions for proper range handling

## [0.1.0] - 2024-03-19

### Added
- Initial release as foundational crate for nusantara-calendar workspace
- `CalendarDate` trait with JDN conversion support
- `CalendarMetadata` trait for calendar information
- `HasAuspiciousness` trait for cultural evaluations
- `CalendarError` enum with comprehensive error types
- `Activity` enum for Indonesian cultural activities
- `AuspiciousnessLevel` enum for 5-level favorability system
- `gregorian_to_jdn()` and `jdn_to_gregorian()` functions
- `stub!` macro for unimplemented features
- Type aliases: `JDN`, `CycleYear`, `SubYearPosition`
- Full documentation with examples
- Comprehensive test suite (10 unit tests + 4 doc tests)
- Platform support for embedded systems and WASM
- Serde serialization support (optional feature)

### Technical Details
- Uses Fliegel & van Flandern (1968) algorithm for JDN conversions
- References Meeus, Jean. *Astronomical Algorithms* (2nd ed.)
- Supports full JDN range (approximately 262,000 BCE to 262,000 CE)
- Implements Gregorian reform anchor (October 15, 1582)
- Includes Sultan Agung epoch (July 8, 1633) for Indonesian calendars

### Documentation
- Complete API documentation with examples
- Cultural context for Indonesian calendar systems
- Performance considerations and guidelines
- Contributing guidelines and source requirements

---

## Version History

### Planned Future Releases

#### [0.2.0] (Planned)
- Additional utility functions for calendar calculations
- Extended cultural activity types
- Performance optimizations for JDN conversions
- Additional error types for specific calendar systems

#### [0.3.0] (Planned)
- Advanced astronomical calculation support
- Extended trait methods for complex calendar operations
- Additional platform optimizations
- Enhanced WASM support

---

## Release Notes

### Version 0.1.0

This initial release establishes the foundation for the nusantara-calendar workspace. The crate provides:

1. **Universal Date System**: Julian Day Number (JDN) as the central pivot for all calendar conversions
2. **Core Traits**: Standardized interfaces for calendar implementations
3. **Cultural Support**: Indonesian-specific features for auspiciousness calculations
4. **Platform Flexibility**: Support for embedded systems, standard library, and WebAssembly

The implementation follows established astronomical algorithms and cultural research to ensure accuracy and authenticity for Indonesian calendar systems.

### Compatibility

- **Minimum Rust Version**: 1.94
- **Platform Support**: Linux, macOS, Windows, WASM, embedded systems
- **Dependency Requirements**: `thiserror` v2.0+, optional `serde` v1.0+
- **License**: Apache-2.0 OR MIT

### Migration Guide

This is the initial release, so no migration is required. Future releases will follow semantic versioning guidelines.

---

## Sources and References

### Algorithm Sources
- Meeus, Jean. *Astronomical Algorithms*, 2nd Edition. Willmann-Blohm, 1998
- Fliegel, H. F. & van Flandern, T. C. (1968). "A Machine Algorithm for Processing Calendar Dates"
- U.S. Naval Observatory JDN conversion algorithms

### Cultural Sources
- Beauducel & Karjanto (2020). *An ethnoarithmetic excursion into the Javanese calendar*
- Various Indonesian cultural and academic sources
- Traditional calendar authorities and practitioners

### Technical References
- Wikipedia: Julian Day Number calculation formulas
- Astronomical Applications Department, U.S. Naval Observatory
- International Astronomical Union standards

---

*For detailed documentation, see the [README.md](README.md) file.*
