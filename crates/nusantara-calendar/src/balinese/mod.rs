//! Balinese calendar module
//!
//! This module provides access to the Balinese Saka calendar system by wrapping
//! the official `balinese-calendar` crate and implementing the `calendar-core` traits.
//!
//! ## Usage
//!
//! ```rust
//! use nusantara_calendar::balinese::BalineseDate;
//! use calendar_core::CalendarDate;
//!
//! let date = BalineseDate::from_ymd(2026, 3, 19).unwrap();
//! println!("Saka year: {}", date.saka_year);
//! ```

// Re-export the official balinese-calendar types
pub use balinese_calendar::{
    BalineseDate as OfficialBalineseDate, BalineseDateError, Pancawara, Saptawara, Sasih, Wuku,
    pawukon, sasih, wariga, wewaran,
};

use crate::{CalendarDate, CalendarError, CalendarMetadata, HasAuspiciousness, JDN};
use calendar_core::{Activity, AuspiciousnessLevel};

/// Balinese calendar date with calendar-core trait implementations
///
/// This is a wrapper around the official `balinese-calendar::BalineseDate` that
/// implements the `calendar-core` traits for interoperability with other calendar systems.
#[derive(Debug, Clone, PartialEq)]
pub struct BalineseDate(pub OfficialBalineseDate);

impl Eq for BalineseDate {}

impl BalineseDate {
    /// Create a Balinese date from Gregorian year, month, and day
    ///
    /// # Arguments
    /// * `year` - Gregorian year
    /// * `month` - Gregorian month (1-12)
    /// * `day` - Gregorian day (1-31)
    ///
    /// # Returns
    /// `Ok(BalineseDate)` if the date is valid, `Err(CalendarError)` otherwise
    ///
    /// # Errors
    /// Returns `CalendarError::OutOfRange` if the Gregorian date is invalid
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Result<Self, CalendarError> {
        OfficialBalineseDate::from_ymd(year, month.into(), day.into())
            .map(BalineseDate)
            .map_err(|_| CalendarError::OutOfRange("Invalid Gregorian date".to_string()))
    }

    /// Get the underlying official `BalineseDate`
    #[must_use]
    pub const fn as_official(&self) -> &OfficialBalineseDate {
        &self.0
    }

    /// Convert from official `BalineseDate`
    #[must_use]
    pub const fn from_official(date: OfficialBalineseDate) -> Self {
        Self(date)
    }
}

impl CalendarDate for BalineseDate {
    fn from_jdn(jdn: JDN) -> Result<Self, CalendarError> {
        Ok(Self(OfficialBalineseDate::from_jdn(jdn)))
    }

    fn to_jdn(&self) -> JDN {
        // Use the official crate's JDN calculation
        self.0.jdn
    }

    fn calendar_name() -> &'static str {
        "Balinese Saka Calendar"
    }

    fn validate_range(&self) -> Result<(), CalendarError> {
        // Use official crate's validation if available, otherwise implement basic range check
        if self.0.gregorian_year < 1800 || self.0.gregorian_year > 2200 {
            return Err(CalendarError::OutOfRange(
                "Year out of supported range".to_string(),
            ));
        }
        Ok(())
    }
}

impl CalendarMetadata for BalineseDate {
    fn epoch() -> JDN {
        // Saka era epoch: proleptic Gregorian 78-03-22 (= Julian 78-03-22)
        // Source: Dershowitz & Reingold, *Calendrical Calculations* (4th ed.), §13.1
        1_749_630
    }

    fn cycle_length() -> Option<calendar_core::CycleYear> {
        // The Balinese Saka year has no fixed year-level cycle.
        // Pawukon (210 days) is a sub-year day cycle, not measured in years.
        None
    }

    fn description() -> &'static str {
        "Traditional Balinese calendar system with Pawukon 210-day cycles, lunar months, and Saka year counting"
    }

    fn cultural_origin() -> &'static str {
        "Balinese Hindu calendar system from Bali, Indonesia, integrating Pawukon cycles with Saka lunar calendar"
    }
}

impl HasAuspiciousness for BalineseDate {
    type Activity = Activity;
    type AuspiciousnessLevel = AuspiciousnessLevel;

    fn auspiciousness_for(&self, _activity: &Self::Activity) -> Self::AuspiciousnessLevel {
        unimplemented!("Balinese auspiciousness_for: not yet implemented")
    }

    fn is_auspicious_day(&self) -> bool {
        unimplemented!("Balinese is_auspicious_day: not yet implemented")
    }
}

// Implement common trait delegations
impl std::ops::Deref for BalineseDate {
    type Target = OfficialBalineseDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<OfficialBalineseDate> for BalineseDate {
    fn from(date: OfficialBalineseDate) -> Self {
        Self(date)
    }
}

#[cfg(test)]
mod balinese_module_tests;
