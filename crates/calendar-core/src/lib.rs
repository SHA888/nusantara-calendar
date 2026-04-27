#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

//! # Nusantara Calendar Core
//!
//! This crate provides the foundational types, traits, and utilities for the
//! nusantara-calendar workspace, which implements traditional Indonesian
//! calendar systems with modern Rust ergonomics.
//!
//! ## Julian Day Number (JDN) Pivot
//!
//! This crate uses the Julian Day Number (JDN) system as the central pivot for
//! all calendar conversions. JDN provides a continuous count of days since
//! noon Universal Time on January 1, 4713 BCE (Julian calendar), making it
//! ideal for converting between different calendar systems.
//!
//! The JDN system allows us to:
//! - Convert any date to/from Gregorian calendar
//! - Implement bidirectional conversions between calendar systems
//! - Perform date arithmetic with consistent results
//! - Handle historical dates across different calendar reforms
//!
//! ## Algorithm Source
//!
//! The Gregorian to JDN conversion algorithm implemented in this crate follows
//! the standard formula from:
//!
//! **Meeus, Jean.** *Astronomical Algorithms*, 2nd Edition.
//! Willmann-Blohm, 1998. Chapter 7: "Julian Day".
//!
//! This reference implementation provides accurate conversions for the full
//! range of historical dates supported by the JDN system (approximately
//! 262,000 BCE to 262,000 CE).
//!
//! ## Core Components
//!
//! ### Types
//! - [`JDN`] - Julian Day Number type alias (`i64`)
//! - [`CycleYear`] - Type for cycle-year fields (`u32`)
//! - [`SubYearPosition`] - Type for sub-year positions (`u8`)
//!
//! ### Traits
//! - [`CalendarDate`] - Core interface for calendar implementations
//! - [`CalendarMetadata`] - Access to calendar metadata and cultural context
//! - [`HasAuspiciousness`] - Auspiciousness calculations for Indonesian calendars
//!
//! ### Error Handling
//! - [`CalendarError`] - Comprehensive error types with detailed context
//! - [`stub!`] macro for marking unimplemented features
//!
//! ### Cultural Features
//! - [`Activity`] - Indonesian cultural activities for auspiciousness evaluation
//! - [`AuspiciousnessLevel`] - Favorability levels for activities and dates
//!
//! ## Platform Support
//!
//! This crate supports multiple compilation targets:
//! - **std**: Standard library with full functionality
//! - **`no_std`**: Embedded systems with `alloc` support
//! - **WASM**: WebAssembly targets for browser usage
//!
//! ## Example Usage
//!
//! ```rust
//! use calendar_core::{gregorian_to_jdn, jdn_to_gregorian, CalendarDate};
//!
//! // Convert Gregorian to JDN
//! let jdn = gregorian_to_jdn(2024, 3, 15);
//!
//! // Convert back to Gregorian
//! let (year, month, day) = jdn_to_gregorian(jdn);
//!
//! // Use with CalendarDate trait implementations
//! // let calendar_date = MyCalendar::from_gregorian(2024, 3, 15)?;
//! ```
//!
//! ## Indonesian Calendar Context
//!
//! This crate is specifically designed to support the rich diversity of
//! Indonesian calendar systems, including:
//! - Javanese calendar (Saka and Islamic integration)
//! - Balinese calendar (Pawukon cycle)
//! - Hijri/Islamic calendar
//! - Chinese calendar integration
//! - Various regional ethnic calendars
//!
//! Each calendar system can implement the core traits while maintaining
//! cultural authenticity and computational accuracy.

extern crate alloc;

// Re-export commonly used items
pub use thiserror::Error;

use alloc::string::String;

// Modules
pub mod auspiciousness;

// Re-export auspiciousness types
pub use auspiciousness::{Activity, AuspiciousnessLevel};

/// Julian Day Number type alias
///
/// Uses i64 to support the full range of historical dates
/// from approximately 262,000 BCE to 262,000 CE
pub type JDN = i64;

/// Type for cycle-year fields (e.g., year in a 60-year cycle)
///
/// Uses u32 to support large cycles while remaining efficient
pub type CycleYear = u32;

/// Type for sub-year positions (e.g., month, day, weekday)
///
/// Uses u8 for compact storage of values 0-255
pub type SubYearPosition = u8;

/// Core error types for calendar operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalendarError {
    /// Date is out of supported range
    OutOfRange(String),
    /// Invalid calendar parameters
    InvalidParameters(String),
    /// Feature not yet implemented
    NotImplemented(String),
    /// Arithmetic error
    ArithmeticError(String),
}

impl core::fmt::Display for CalendarError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfRange(msg) => write!(f, "Date out of supported range: {msg}"),
            Self::InvalidParameters(msg) => write!(f, "Invalid calendar parameters: {msg}"),
            Self::NotImplemented(msg) => write!(f, "Feature not yet implemented: {msg}"),
            Self::ArithmeticError(msg) => write!(f, "Arithmetic error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CalendarError {}

/// Trait for calendar date implementations
///
/// This trait defines the core interface that all calendar systems
/// in the nusantara-calendar workspace must implement. It provides
/// bidirectional conversion with Julian Day Numbers and Gregorian dates,
/// serving as the foundation for all calendar implementations.
///
/// # Required Implementations
///
/// Calendar types must implement:
/// - [`Self::from_jdn()`] - Convert from JDN to calendar date
/// - [`Self::to_jdn()`] - Convert from calendar date to JDN
/// - [`Self::calendar_name()`] - Return the calendar system name
/// - [`Self::validate_range()`] - Check if date is within supported range
///
/// # Provided Implementations
///
/// Default implementations are provided for:
/// - [`Self::from_gregorian()`] - Convert via JDN intermediate
/// - [`Self::to_gregorian()`] - Convert via JDN intermediate
///
/// # Implementation Guidelines
///
/// When implementing this trait:
///
/// 1. **Use JDN as the canonical representation** - All conversions should
///    go through JDN to ensure consistency across calendar systems
///
/// 2. **Validate date ranges** - Each calendar should define reasonable
///    bounds and return appropriate errors for out-of-range dates
///
/// 3. **Handle cultural specifics** - Account for calendar-specific
///    rules like leap months, intercalary days, etc.
///
/// 4. **Provide clear error messages** - Use descriptive error messages
///    that help users understand validation failures
///
/// # Example Implementation
///
/// ```rust
/// use calendar_core::{CalendarDate, CalendarError, JDN};
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct MyCalendarDate {
///     year: i32,
///     month: u8,
///     day: u8,
/// }
///
/// impl CalendarDate for MyCalendarDate {
///     fn from_jdn(jdn: JDN) -> Result<Self, CalendarError> {
///         // Convert JDN to calendar date
///         // Implementation depends on calendar rules
///         todo!("Implement JDN to calendar conversion")
///     }
///
///     fn to_jdn(&self) -> JDN {
///         // Convert calendar date to JDN
///         // Implementation depends on calendar rules
///         todo!("Implement calendar to JDN conversion")
///     }
///
///     fn calendar_name() -> &'static str {
///         "My Calendar System"
///     }
///
///     fn validate_range(&self) -> Result<(), CalendarError> {
///         // Check if date is within supported range
///         if self.year < 1 || self.year > 9999 {
///             return Err(CalendarError::OutOfRange(
///                 "Year must be between 1 and 9999".to_string()
///             ));
///         }
///         Ok(())
///     }
/// }
/// ```
///
/// # Performance Considerations
///
/// - JDN conversions are the most computationally expensive operations
/// - Cache results when performing repeated conversions
/// - Use the provided Gregorian conversion methods for convenience
/// - Consider lazy evaluation for complex calendar calculations
pub trait CalendarDate: Clone + PartialEq + Eq + core::fmt::Debug {
    /// Convert from Julian Day Number to this calendar's date
    ///
    /// This is the core conversion method that all calendar implementations
    /// must provide. It should handle the specific rules and calculations
    /// required for the calendar system.
    ///
    /// # Arguments
    /// * `jdn` - Julian Day Number to convert
    ///
    /// # Returns
    /// - `Ok(date)` - Successfully converted calendar date
    /// - `Err(CalendarError)` - Conversion failed (invalid JDN, out of range, etc.)
    ///
    /// # Errors
    /// Returns `CalendarError` if the JDN is invalid or outside the supported range.
    ///
    /// # Implementation Notes
    ///
    /// - Should handle edge cases like epoch dates, leap years, etc.
    /// - Must return appropriate errors for invalid inputs
    /// - Should be the inverse of [`Self::to_jdn()`] for valid dates
    fn from_jdn(jdn: JDN) -> Result<Self, CalendarError>
    where
        Self: Sized;

    /// Convert from this calendar's date to Julian Day Number
    ///
    /// This method should provide the exact inverse of [`Self::from_jdn()`]
    /// for all valid dates in the calendar's supported range.
    ///
    /// # Returns
    /// Julian Day Number representing this calendar date
    ///
    /// # Implementation Notes
    ///
    /// - Must handle calendar-specific rules (leap months, etc.)
    /// - Should return consistent results for the same date
    /// - Round-trip with [`Self::from_jdn()`] should preserve the original date
    fn to_jdn(&self) -> JDN;

    /// Get the calendar system name
    ///
    /// Returns a human-readable name for the calendar system,
    /// suitable for display in user interfaces and documentation.
    ///
    /// # Returns
    /// String slice containing the calendar name
    ///
    /// # Examples
    ///
    /// - "Gregorian"
    /// - "Javanese Saka"
    /// - "Islamic Hijri"
    /// - "Chinese Lunisolar"
    fn calendar_name() -> &'static str;

    /// Validate that this date is within the supported range
    ///
    /// Each calendar system should define reasonable bounds for
    /// valid dates and return appropriate errors for out-of-range values.
    ///
    /// # Returns
    /// - `Ok(())` - Date is valid and within range
    /// - `Err(CalendarError)` - Date is invalid or out of supported range
    ///
    /// # Errors
    /// Returns `CalendarError` if the date is outside the supported range.
    ///
    /// # Implementation Guidelines
    ///
    /// - Check year ranges based on historical accuracy
    /// - Validate month and day values for the specific calendar
    /// - Consider astronomical constraints (e.g., full moon dates)
    /// - Provide clear error messages for validation failures
    fn validate_range(&self) -> Result<(), CalendarError>;

    /// Convert from Gregorian date to this calendar's date
    ///
    /// Default implementation uses JDN as intermediate format:
    /// Gregorian -> JDN -> Calendar Date
    ///
    /// # Arguments
    /// * `year` - Gregorian year (CE, can be negative for BCE)
    /// * `month` - Gregorian month (1-12)
    /// * `day` - Gregorian day (1-31, depending on month)
    ///
    /// # Returns
    /// - `Ok(date)` - Successfully converted calendar date
    /// - `Err(CalendarError)` - Conversion failed
    ///
    /// # Errors
    /// Returns `CalendarError` if the conversion fails.
    ///
    /// # Performance
    ///
    /// This involves two conversions: Gregorian->JDN and JDN->Calendar.
    /// For performance-critical applications, consider implementing
    /// direct Gregorian->Calendar conversion if the calendar system
    /// has a known relationship with the Gregorian calendar.
    fn from_gregorian(year: i32, month: u8, day: u8) -> Result<Self, CalendarError>
    where
        Self: Sized,
    {
        let jdn = gregorian_to_jdn(year, month, day);
        Self::from_jdn(jdn)
    }

    /// Convert from this calendar's date to Gregorian date
    ///
    /// Default implementation uses JDN as intermediate format:
    /// Calendar Date -> JDN -> Gregorian
    ///
    /// # Returns
    /// Tuple of (year, month, day) in Gregorian calendar
    ///
    /// # Performance
    ///
    /// This involves two conversions: Calendar->JDN and JDN->Gregorian.
    /// For performance-critical applications, consider implementing
    /// direct Calendar->Gregorian conversion if possible.
    fn to_gregorian(&self) -> (i32, u8, u8) {
        jdn_to_gregorian(self.to_jdn())
    }
}

/// Trait for calendar metadata and information
///
/// This trait provides access to calendar-specific metadata such as
/// epoch information, cycle information, and cultural context. Implementers
/// should provide accurate historical and cultural information about their
/// calendar systems.
///
/// # Implementation Requirements
///
/// Calendar implementations must:
/// - Provide accurate epoch dates with historical sources
/// - Document cycle lengths with cultural references
/// - Include cultural origin information
/// - Optionally provide reference sources for verification
///
/// # Reference Sources Contract
///
/// Implementers of this trait should document their reference sources for:
/// - Epoch dates and historical accuracy
/// - Cycle calculations and astronomical basis
/// - Cultural practices and calendar rules
/// - Regional variations and historical changes
///
/// Recommended reference sources include:
/// - Historical astronomical records
/// - Cultural and religious texts
/// - Academic research on calendar systems
/// - Government or institutional standards
///
/// # Example
///
/// ```rust
/// use calendar_core::{CalendarMetadata, JDN};
///
/// struct MyCalendar;
///
/// impl CalendarMetadata for MyCalendar {
///     fn epoch() -> JDN {
///         // Epoch based on historical records
///         1948439 // Example: March 22, 2024 CE
///     }
///
///     fn cycle_length() -> Option<u32> {
///         Some(60) // 60-year cycle common in Indonesian calendars
///     }
///
///     fn description() -> &'static str {
///         "Traditional Indonesian calendar with 60-year cycle"
///     }
///
///     fn cultural_origin() -> &'static str {
///         "Javanese court calendar system, integrated with Islamic calendar"
///     }
/// }
/// ```
pub trait CalendarMetadata {
    /// Get the epoch (starting date) for this calendar
    ///
    /// The epoch represents the starting point of the calendar system,
    /// typically corresponding to a historically significant date.
    ///
    /// # Returns
    /// Julian Day Number of the calendar's epoch
    ///
    /// # Historical Context
    ///
    /// Implementers should base epoch dates on reliable historical sources
    /// and document any uncertainties or variations in historical records.
    fn epoch() -> JDN;

    /// Get the cycle length if this calendar uses cycles
    ///
    /// Many Indonesian calendars use cyclical systems (e.g., 60-year cycles).
    /// This method returns the length of such cycles if applicable.
    ///
    /// # Returns
    /// - `Some(length)` - Number of years in the cycle
    /// - `None` - Calendar doesn't use year cycles
    ///
    /// # Examples
    ///
    /// - Javanese calendar: 60-year cycle
    /// - Chinese calendar: 60-year cycle
    /// - Gregorian calendar: No cycle (returns `None`)
    #[must_use]
    fn cycle_length() -> Option<CycleYear> {
        None
    }

    /// Get a description of this calendar system
    ///
    /// Provides a concise description of the calendar system, including
    /// its main characteristics and usage context.
    ///
    /// # Returns
    /// String slice describing the calendar system
    fn description() -> &'static str;

    /// Get the cultural/ethnic origin of this calendar
    ///
    /// Identifies the cultural, ethnic, or religious group that
    /// developed and primarily uses this calendar system.
    ///
    /// # Returns
    /// String slice identifying the cultural origin
    ///
    /// # Examples
    ///
    /// - "Javanese court calendar"
    /// - "Balinese Pawukon system"
    /// - "Islamic Hijri calendar"
    /// - "Chinese lunisolar calendar"
    fn cultural_origin() -> &'static str;
}

/// Trait for calendars that have auspiciousness calculations
///
/// This trait enables Indonesian calendar systems to provide cultural
/// auspiciousness evaluations for various activities. Many Indonesian
/// traditional calendars include concepts of auspicious and inauspicious
/// days that influence important life events and activities.
///
/// # Cultural Context
///
/// Indonesian calendar systems often incorporate auspiciousness concepts
/// from various cultural and religious traditions:
///
/// - **Javanese calendar**: Balancing elements, market days, and spiritual aspects
/// - **Balinese calendar**: Complex system of auspicious/inauspicious days
/// - **Chinese integration**: Feng shui and zodiac considerations
/// - **Islamic integration**: Religious observances and favorable timing
///
/// # Implementation Requirements
///
/// Calendar systems implementing this trait should:
///
/// 1. **Define Activity Types**: Specify which cultural activities are relevant
/// 2. **Provide Auspiciousness Levels**: Use the standard 5-level system
/// 3. **Implement Cultural Logic**: Apply authentic cultural rules and calculations
/// 4. **Document Sources**: Reference cultural texts or expert knowledge
///
/// # Example Implementation
///
/// ```rust
/// use calendar_core::{HasAuspiciousness, Activity, AuspiciousnessLevel};
///
/// struct MyCalendarDate {
///     // Calendar date fields
/// }
///
/// impl HasAuspiciousness for MyCalendarDate {
///     type Activity = Activity;
///     type AuspiciousnessLevel = AuspiciousnessLevel;
///
///     fn auspiciousness_for(&self, activity: &Activity) -> Self::AuspiciousnessLevel {
///         match activity {
///             Activity::Marriage => {
///                 // Check if this date is auspicious for marriage
///                 // Based on cultural rules and calculations
///                 // Implementation would go here
///                 AuspiciousnessLevel::Auspicious // Placeholder
///             }
///             Activity::Building => {
///                 // Check auspiciousness for construction
///                 AuspiciousnessLevel::Neutral // Placeholder
///             }
///             // Handle other activities...
///             _ => AuspiciousnessLevel::Neutral,
///         }
///     }
///
///     fn is_auspicious_day(&self) -> bool {
///         // General auspiciousness for the day
///         // Implementation would calculate daily auspiciousness
///         true // Placeholder
///     }
/// }
/// ```
///
/// # Performance Considerations
///
/// - Auspiciousness calculations can be computationally intensive
/// - Consider caching results for frequently accessed dates
/// - Some calculations may depend on complex astronomical data
/// - Balance accuracy with performance for real-time applications
pub trait HasAuspiciousness {
    /// Activity types that can be evaluated for auspiciousness
    ///
    /// This associated type defines which activities the calendar system
    /// can evaluate for auspiciousness. Most implementations will use
    /// the standard [`Activity`] enum, but custom calendars may define
    /// their own activity types.
    ///
    /// # Recommended Types
    ///
    /// - Use the standard [`Activity`] enum for common Indonesian activities
    /// - Extend with custom activities for specific calendar systems
    /// - Consider cultural relevance when defining activity sets
    type Activity;

    /// Auspiciousness levels
    ///
    /// This associated type defines the levels of auspiciousness that
    /// the calendar system can return. Most implementations should use
    /// the standard [`AuspiciousnessLevel`] enum for consistency.
    ///
    /// # Standard Levels
    ///
    /// - `VeryAuspicious` - Extremely favorable for the activity
    /// - `Auspicious` - Favorable for the activity
    /// - `Neutral` - Neither favorable nor unfavorable
    /// - `Inauspicious` - Unfavorable for the activity
    /// - `VeryInauspicious` - Extremely unfavorable for the activity
    type AuspiciousnessLevel;

    /// Determine the auspiciousness level for a given activity
    ///
    /// This method evaluates how auspicious a specific date is for
    /// performing a particular activity according to the calendar system's
    /// cultural rules and traditions.
    ///
    /// # Arguments
    /// * `activity` - The activity to evaluate for auspiciousness
    ///
    /// # Returns
    /// Auspiciousness level for the given activity on this date
    ///
    /// # Implementation Guidelines
    ///
    /// - Apply authentic cultural rules and calculations
    /// - Consider multiple factors (planetary positions, market days, etc.)
    /// - Handle edge cases (conflicting auspiciousness indicators)
    /// - Document the cultural basis for the calculations
    ///
    /// # Cultural Factors to Consider
    ///
    /// - **Astronomical**: Planetary positions, lunar phases
    /// - **Cyclical**: Market days, elemental cycles
    /// - **Religious**: Holy days, prayer times
    /// - **Cultural**: Traditional beliefs, regional variations
    fn auspiciousness_for(&self, activity: &Self::Activity) -> Self::AuspiciousnessLevel;

    /// Check if a day is generally auspicious
    ///
    /// Provides a quick assessment of whether the day is generally
    /// considered auspicious for most activities. This is useful for
    /// applications that need a simple good/bad day assessment.
    ///
    /// # Returns
    /// `true` if the day is generally auspicious, `false` otherwise
    ///
    /// # Implementation Notes
    ///
    /// - Should consider the overall auspiciousness of the day
    /// - May combine multiple factors into a single assessment
    /// - Useful for calendar displays and quick filters
    /// - Should be consistent with detailed activity-specific evaluations
    ///
    /// # Default Logic
    ///
    /// A common approach is to consider a day auspicious if:
    /// - It's neutral or better for most common activities
    /// - It doesn't have major inauspicious indicators
    /// - It aligns with favorable astronomical or cultural conditions
    fn is_auspicious_day(&self) -> bool;
}

/// Macro for stub implementations
///
/// Used to mark features that are not yet implemented but
/// have defined interfaces.
#[macro_export]
macro_rules! stub {
    ($msg:expr) => {
        return Err($crate::CalendarError::NotImplemented(
            ::alloc::string::ToString::to_string(&$msg),
        ))
    };
}

/// Basic Gregorian to Julian Day Number conversion
///
/// Implements the standard Gregorian calendar algorithm
/// from the Julian Day Number Wikipedia page and astronomical sources.
///
/// # Arguments
/// * `year` - Gregorian year (CE, can be negative for BCE)
/// * `month` - Gregorian month (1-12)
/// * `day` - Gregorian day (1-31, depending on month)
///
/// # Returns
/// Julian Day Number for the given Gregorian date
#[must_use]
pub fn gregorian_to_jdn(year: i32, month: u8, day: u8) -> JDN {
    let (y, m) = if month <= 2 {
        (year - 1, month + 12)
    } else {
        (year, month)
    };

    // Algorithm from: <https://en.wikipedia.org/wiki/Julian_day#Calculation>
    // Cast m and day to i32 to avoid type issues
    let m_i32 = i32::from(m);
    let day_i32 = i32::from(day);

    let jdn = (1461 * (y + 4800 + (m_i32 - 14) / 12)) / 4
        + (367 * (m_i32 - 2 - 12 * ((m_i32 - 14) / 12))) / 12
        - (3 * ((y + 4900 + (m_i32 - 14) / 12) / 100)) / 4
        + day_i32
        - 32075;

    JDN::from(jdn)
}

/// Basic Julian Day Number to Gregorian conversion
///
/// Implements the canonical Fliegel & van Flandern (1968) algorithm
/// from the U.S. Naval Observatory, which is the authoritative inverse
/// of the Gregorian to JDN conversion.
///
/// # Arguments
/// * `jdn` - Julian Day Number
///
/// # Returns
/// Tuple of (year, month, day) in Gregorian calendar
///
/// # Panics
/// Panics if JDN is outside the supported range for Gregorian conversion
/// (approximately -2,147,483,648 to 2,147,483,647)
///
/// # Reference
/// Fliegel, H. F. & van Flandern, T. C. 1968, "A Machine Algorithm for
/// Processing Calendar Dates", Communications of the ACM, 11, 657.
/// [https://aa.usno.navy.mil/faq/JD_formula](https://aa.usno.navy.mil/faq/JD_formula)
#[must_use]
pub fn jdn_to_gregorian(jdn: JDN) -> (i32, u8, u8) {
    // Validate JDN is within i32 range to prevent overflow
    assert!(
        jdn >= JDN::from(i32::MIN) && jdn <= JDN::from(i32::MAX),
        "JDN {jdn} is outside supported range for Gregorian conversion ({} to {})",
        i32::MIN,
        i32::MAX
    );

    // Fliegel & van Flandern algorithm (1968), all intermediates in i64
    // to avoid overflow for JDNs in the upper half of the i32 range.
    // Reference: U.S. Naval Observatory
    let l = jdn + 68_569_i64;
    let n = (4 * l) / 146_097_i64;
    let l = l - (146_097 * n + 3) / 4;
    let i = (4_000 * (l + 1)) / 1_461_001_i64;
    let l = l - (1_461 * i) / 4 + 31;
    let j = (80 * l) / 2_447_i64;
    let day = l - (2_447 * j) / 80;
    let l = j / 11;
    let month = j + 2 - 12 * l;
    let year = 100 * (n - 49) + i + l;

    // Validate calculated values fit in u8 range
    debug_assert!((1..=31).contains(&day), "Invalid day calculation: {day}");
    debug_assert!(
        (1..=12).contains(&month),
        "Invalid month calculation: {month}"
    );

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    {
        (year as i32, month as u8, day as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_gregorian_to_jdn_reform_anchor() {
        // Gregorian reform anchor: October 15, 1582
        let jdn = gregorian_to_jdn(1582, 10, 15);
        assert_eq!(jdn, 2_299_161);
    }

    #[test]
    fn test_gregorian_to_jdn_sultan_agung_epoch() {
        // Sultan Agung epoch: July 8, 1633
        let jdn = gregorian_to_jdn(1633, 7, 8);
        assert_eq!(jdn, 2_317_690);
    }

    #[test]
    fn test_jdn_to_gregorian_reform_anchor() {
        // Test inverse of reform anchor
        let (year, month, day) = jdn_to_gregorian(2_299_161);
        assert_eq!((year, month, day), (1582, 10, 15));
    }

    #[test]
    fn test_jdn_to_gregorian_sultan_agung_epoch() {
        // Test inverse of Sultan Agung epoch
        let (year, month, day) = jdn_to_gregorian(2_317_690);
        assert_eq!((year, month, day), (1633, 7, 8));
    }

    #[test]
    fn test_round_trip_conversions() {
        // Test round-trip property for various dates
        let test_dates = [
            (2000, 1, 1),    // Y2K
            (2024, 2, 29),   // Leap day
            (1900, 3, 1),    // Non-leap year century
            (1600, 1, 1),    // Leap year century
            (1582, 10, 4),   // Last Julian day
            (1582, 10, 15),  // First Gregorian day
            (1, 1, 1),       // Early date
            (-4713, 11, 24), // JDN epoch
        ];

        for (year, month, day) in test_dates {
            let jdn = gregorian_to_jdn(year, month, day);
            let (year2, month2, day2) = jdn_to_gregorian(jdn);
            assert_eq!(
                (year, month, day),
                (year2, month2, day2),
                "Round-trip failed for {year}-{month}-{day}"
            );
        }
    }

    #[test]
    fn test_stub_macro() {
        // Test that stub! returns the correct error
        fn test_function() -> Result<(), CalendarError> {
            stub!("test message");
        }

        let result = test_function();
        assert!(result.is_err());
        match result.unwrap_err() {
            CalendarError::NotImplemented(msg) => {
                assert_eq!(msg, "test message");
            }
            _ => panic!("Expected NotImplemented error"),
        }
    }

    #[test]
    fn test_calendar_error_display() {
        let error = CalendarError::OutOfRange("test range".to_string());
        assert_eq!(error.to_string(), "Date out of supported range: test range");

        let error = CalendarError::InvalidParameters("test params".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid calendar parameters: test params"
        );

        let error = CalendarError::NotImplemented("test feature".to_string());
        assert_eq!(
            error.to_string(),
            "Feature not yet implemented: test feature"
        );

        let error = CalendarError::ArithmeticError("test math".to_string());
        assert_eq!(error.to_string(), "Arithmetic error: test math");
    }

    #[test]
    fn test_activity_enum() {
        let activity = Activity::Marriage;
        assert!(activity.description().contains("Marriage"));

        let custom = Activity::Custom("Custom activity".to_string());
        assert_eq!(custom.description(), "Custom activity");
    }

    #[test]
    fn test_auspiciousness_level_enum() {
        let level = AuspiciousnessLevel::Auspicious;
        assert!(level.is_auspicious());
        assert!(!level.is_very_auspicious());
        assert!(!level.is_inauspicious());
        assert!(!level.is_very_inauspicious());
        assert!(!level.is_neutral());

        let level = AuspiciousnessLevel::VeryAuspicious;
        assert!(level.is_auspicious());
        assert!(level.is_very_auspicious());
        assert!(!level.is_inauspicious());
        assert!(!level.is_neutral());

        let level = AuspiciousnessLevel::Neutral;
        assert!(!level.is_auspicious());
        assert!(!level.is_inauspicious());
        assert!(!level.is_very_auspicious());
        assert!(!level.is_very_inauspicious());
        assert!(level.is_neutral());

        let level = AuspiciousnessLevel::Inauspicious;
        assert!(!level.is_auspicious());
        assert!(level.is_inauspicious());
        assert!(!level.is_very_inauspicious());
        assert!(!level.is_neutral());

        let level = AuspiciousnessLevel::VeryInauspicious;
        assert!(!level.is_auspicious());
        assert!(level.is_inauspicious());
        assert!(level.is_very_inauspicious());
        assert!(!level.is_neutral());
    }

    #[test]
    fn test_type_definitions() {
        // Test that type aliases work correctly
        let jdn: JDN = 2_451_545;
        let cycle_year: CycleYear = 60;
        let sub_year: SubYearPosition = 12;

        assert_eq!(jdn, 2_451_545);
        assert_eq!(cycle_year, 60);
        assert_eq!(sub_year, 12);
    }
}
