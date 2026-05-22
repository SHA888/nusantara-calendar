//! Hijriyah (Islamic) calendar system
//!
//! Independent implementation of the tabular Islamic calendar using the Hijri lunar calendar.
//! Supports years 1–1600 AH (622–2178 CE), spanning the full historical and practical range.
//!
//! ## Algorithm Sources
//!
//! - Dershowitz & Reingold, *Calendrical Calculations* (4th ed.), Chapter 6 — Islamic Calendar
//! - Meeus, *Astronomical Algorithms* (2nd ed.), Chapter 9 — Islamic Calendar (cross-validation)
//!
//! ## Calendar Summary
//!
//! The Hijri calendar is a lunar calendar consisting of:
//! - 12 lunar months (each 29 or 30 days)
//! - A year of approximately 354.36667 days
//! - A 30-year cycle with 11 leap years
//! - Leap years: 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29 in each 30-year cycle
//!
//! Month names and their lengths:
//! 1. Muharram — 30 days
//! 2. Safar — 29 days
//! 3. Rabi' al-awwal — 30 days
//! 4. Rabi' al-thani — 29 days
//! 5. Jumada al-awwal — 30 days
//! 6. Jumada al-thani — 29 days
//! 7. Rajab — 30 days
//! 8. Sha'ban — 29 days
//! 9. Ramadan — 30 days
//! 10. Shawwal — 29 days
//! 11. Dhu al-Qi'dah — 30 days
//! 12. Dhu al-Hijjah — 29 days (30 in leap years)

use crate::{CalendarDate, CalendarError, CalendarMetadata, JDN};

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::ToString;

// ============================================================================
// EPOCH AND RANGE CONSTANTS
// ============================================================================

/// Islamic calendar epoch: 1 Muharram 1 AH = July 16, 622 CE (Julian)
///
/// Sources: Dershowitz & Reingold (2018), Ch. 6, Eq. 6.1;
///          Meeus (1998), Ch. 9, p. 73.
pub const HIJRI_EPOCH_JDN: i64 = 1_948_439;

/// Minimum supported Hijri year (year 1 AH)
pub const HIJRI_YEAR_MIN: u32 = 1;

/// Maximum supported Hijri year (year 1600 AH)
pub const HIJRI_YEAR_MAX: u32 = 1600;

/// Minimum supported JDN (year 1 AH)
pub const JDN_MIN: i64 = HIJRI_EPOCH_JDN;

/// Maximum supported JDN: 30 Dhu al-Hijjah 1600 AH (last day of year 1600).
/// = `HIJRI_EPOCH_JDN + 354*1599 + floor((3+11*1600)/30) + 325 + 29` = 2515425
pub const JDN_MAX: i64 = 2_515_425;

// ============================================================================
// MONTH DATA
// ============================================================================

/// Days in each month for a common (non-leap) year
/// Odd-indexed months: 30 days; even-indexed months: 29 days
/// Month 12 (Dhu al-Hijjah) has 29 days in common years, 30 in leap years
pub const MONTH_DAYS_COMMON: [u8; 12] = [30, 29, 30, 29, 30, 29, 30, 29, 30, 29, 30, 29];

/// Days in each month for a leap year
/// Only month 12 (Dhu al-Hijjah) differs: 30 days instead of 29
pub const MONTH_DAYS_LEAP: [u8; 12] = [30, 29, 30, 29, 30, 29, 30, 29, 30, 29, 30, 30];

/// Hijri month names (transliterated to Arabic-influenced English)
pub const MONTH_NAMES: [&str; 12] = [
    "Muharram",
    "Safar",
    "Rabi' al-awwal",
    "Rabi' al-thani",
    "Jumada al-awwal",
    "Jumada al-thani",
    "Rajab",
    "Sha'ban",
    "Ramadan",
    "Shawwal",
    "Dhu al-Qi'dah",
    "Dhu al-Hijjah",
];

// ============================================================================
// LEAP YEAR LOGIC
// ============================================================================

/// Check if a Hijri year is a leap year
///
/// Uses the canonical D-R leap year rule: `(14 + 11*year) mod 30 < 11`.
/// This places leap years at positions 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29 in each
/// 30-year cycle, adding 11 leap days every 30 years (average year = 354.36667 days).
///
/// # Arguments
/// * `year` - Hijri year (1–1600)
///
/// # Returns
/// `true` if the year is a leap year (has 355 days), `false` otherwise
///
/// # Sources
/// - Dershowitz & Reingold (2018), Ch. 6, Eq. 6.7 (`islamic-leap-year?`)
#[must_use]
pub const fn is_leap_year(year: u32) -> bool {
    (14 + 11 * year as u64) % 30 < 11
}

/// Count leap years from year 1 up to (but not including) the given year
///
/// Returns the number of leap years in years 1 through (year - 1), inclusive.
/// Uses the closed-form D-R expression `floor((3 + 11*Y) / 30)`, which is equivalent
/// to enumerating positions 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29 in each 30-year cycle.
///
/// # Sources
/// - Dershowitz & Reingold (2018), Ch. 6, Eq. 6.8 (`fixed-from-islamic`, leap correction)
#[must_use]
pub const fn count_leap_years_before(year: u32) -> u32 {
    // For year ≤ HIJRI_YEAR_MAX (1600), 11 * year + 3 ≤ 17603, well within u32.
    (3 + 11 * year) / 30
}

// ============================================================================
// CORE CONVERSION FUNCTIONS
// ============================================================================

/// Get the number of days in a Hijri month
///
/// # Arguments
/// * `month` - Month number (1–12)
/// * `year` - Hijri year (to determine if leap year for Dhu al-Hijjah)
///
/// # Returns
/// Number of days (29 or 30)
#[must_use]
pub const fn days_in_month(month: u8, year: u32) -> u8 {
    if month < 1 || month > 12 {
        0
    } else {
        let month_idx = (month - 1) as usize;
        if is_leap_year(year) {
            MONTH_DAYS_LEAP[month_idx]
        } else {
            MONTH_DAYS_COMMON[month_idx]
        }
    }
}

/// Get the number of days in a Hijri year
///
/// # Arguments
/// * `year` - Hijri year
///
/// # Returns
/// 354 days for common years, 355 days for leap years
#[must_use]
pub const fn days_in_year(year: u32) -> u16 {
    if is_leap_year(year) { 355 } else { 354 }
}

/// Convert Hijri date to Julian Day Number
///
/// Implements the tabular algorithm from Dershowitz & Reingold, Chapter 6, Equation 6.2.
///
/// # Arguments
/// * `year` - Hijri year (1–1600)
/// * `month` - Month (1–12)
/// * `day` - Day of month (1–30)
///
/// # Returns
/// Julian Day Number
///
/// # Algorithm
/// Days since epoch = (days in previous years) + (days in previous months) + (day - 1)
/// - Days in previous years: 354 * (year - 1) + (leap years before this year)
/// - Days in previous months: sum of days in months 1 to (month - 1)
///
/// # Sources
/// - Dershowitz & Reingold (2018), Ch. 6, Eq. 6.2
#[must_use]
pub const fn hijri_to_jdn(year: u32, month: u8, day: u8) -> i64 {
    // Days in all previous complete years
    let days_in_years = 354_i64 * (year as i64 - 1);

    // Leap years added to previous years
    let leap_years = count_leap_years_before(year) as i64;

    // Days in all previous complete months this year
    let mut days_in_months = 0_i64;
    let mut m = 1;
    while m < month {
        days_in_months += days_in_month(m, year) as i64;
        m += 1;
    }

    // Combine all components
    let total_days = days_in_years + leap_years + days_in_months + (day as i64) - 1;
    HIJRI_EPOCH_JDN + total_days
}

/// Convert Julian Day Number to Hijri date
///
/// Implements the inverse tabular algorithm from Dershowitz & Reingold, Ch. 6,
/// Eq. 6.9 (`islamic-from-fixed`). Closed-form, no iteration required.
///
/// # Arguments
/// * `jdn` - Julian Day Number
///
/// # Returns
/// Result containing (year, month, day) tuple, or error if JDN is out of range
///
/// # Algorithm
/// ```text
/// Y = floor((30 * (jdn - epoch) + 10646) / 10631)
/// prior_days = jdn - hijri_to_jdn(Y, 1, 1)
/// M = floor((11 * prior_days + 330) / 325)
/// D = 1 + jdn - hijri_to_jdn(Y, M, 1)
/// ```
///
/// # Errors
/// Returns `CalendarError::OutOfRange` if JDN is outside supported range
///
/// # Sources
/// - Dershowitz & Reingold (2018), Ch. 6, Eq. 6.9
pub fn jdn_to_hijri(jdn: i64) -> Result<(u32, u8, u8), CalendarError> {
    if !(JDN_MIN..=JDN_MAX).contains(&jdn) {
        let msg = {
            #[cfg(feature = "std")]
            {
                format!("JDN {jdn} outside supported range {JDN_MIN}–{JDN_MAX}")
            }
            #[cfg(not(feature = "std"))]
            {
                "JDN outside supported range".to_string()
            }
        };
        return Err(CalendarError::OutOfRange(msg));
    }

    // D-R closed-form year estimation
    let year_i64 = (30 * (jdn - HIJRI_EPOCH_JDN) + 10646) / 10631;
    let year = u32::try_from(year_i64).map_err(|_| {
        CalendarError::ArithmeticError("Computed year does not fit in u32".to_string())
    })?;

    // Days elapsed from start of this Hijri year
    let prior_days = jdn - hijri_to_jdn(year, 1, 1);

    // D-R closed-form month estimation
    let month_i64 = (11 * prior_days + 330) / 325;
    let month = u8::try_from(month_i64).map_err(|_| {
        CalendarError::ArithmeticError("Computed month does not fit in u8".to_string())
    })?;

    // Day within the month
    let day_i64 = 1 + jdn - hijri_to_jdn(year, month, 1);
    let day = u8::try_from(day_i64).map_err(|_| {
        CalendarError::ArithmeticError("Computed day does not fit in u8".to_string())
    })?;

    Ok((year, month, day))
}

// ============================================================================
// HIJRI DATE TYPE
// ============================================================================

/// A date in the Hijri (Islamic) calendar system
///
/// Represents a lunar date in the Islamic calendar with year, month, and day.
/// The calendar is purely lunar with no intercalation to the solar year.
///
/// # Sources
/// - Dershowitz & Reingold (2018), *Calendrical Calculations*, 4th ed., Chapter 6
/// - Meeus (1998), *Astronomical Algorithms*, 2nd ed., Chapter 9
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HijriDate {
    /// Year in the Hijri calendar (1–1600 AH)
    pub year: u32,
    /// Month (1–12, from Muharram to Dhu al-Hijjah)
    pub month: u8,
    /// Day of month (1–30)
    pub day: u8,
}

impl HijriDate {
    /// Create a `HijriDate` from year, month, and day
    ///
    /// # Arguments
    /// * `year` - Hijri year (1–1600)
    /// * `month` - Month (1–12)
    /// * `day` - Day (1–30)
    ///
    /// # Returns
    /// `Some(HijriDate)` if valid, `None` otherwise
    #[must_use]
    pub const fn new(year: u32, month: u8, day: u8) -> Option<Self> {
        if year < HIJRI_YEAR_MIN || year > HIJRI_YEAR_MAX {
            return None;
        }
        if month < 1 || month > 12 {
            return None;
        }
        if day < 1 || day > 30 {
            return None;
        }
        Some(Self { year, month, day })
    }

    /// Get the month name (transliterated)
    #[must_use]
    pub const fn month_name(&self) -> &'static str {
        if self.month < 1 || self.month > 12 {
            "Unknown"
        } else {
            MONTH_NAMES[(self.month - 1) as usize]
        }
    }

    /// Check if this is a leap year
    #[must_use]
    pub const fn is_leap_year(&self) -> bool {
        is_leap_year(self.year)
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS
// ============================================================================

impl CalendarDate for HijriDate {
    fn from_jdn(jdn: JDN) -> Result<Self, CalendarError> {
        let (year, month, day) = jdn_to_hijri(jdn)?;
        Ok(Self { year, month, day })
    }

    fn to_jdn(&self) -> JDN {
        hijri_to_jdn(self.year, self.month, self.day)
    }

    fn calendar_name() -> &'static str {
        "Islamic Hijri Calendar"
    }

    fn validate_range(&self) -> Result<(), CalendarError> {
        if self.year < HIJRI_YEAR_MIN || self.year > HIJRI_YEAR_MAX {
            let msg = {
                #[cfg(feature = "std")]
                {
                    format!(
                        "Hijri year {0} outside range {1}–{2}",
                        self.year, HIJRI_YEAR_MIN, HIJRI_YEAR_MAX
                    )
                }
                #[cfg(not(feature = "std"))]
                {
                    "Hijri year outside supported range".to_string()
                }
            };
            return Err(CalendarError::OutOfRange(msg));
        }
        if self.month < 1 || self.month > 12 {
            let msg = {
                #[cfg(feature = "std")]
                {
                    format!("Month {0} outside range 1–12", self.month)
                }
                #[cfg(not(feature = "std"))]
                {
                    "Invalid month".to_string()
                }
            };
            return Err(CalendarError::InvalidParameters(msg));
        }
        let max_day = days_in_month(self.month, self.year);
        if self.day < 1 || self.day > max_day {
            let msg = {
                #[cfg(feature = "std")]
                {
                    format!("Day {0} outside range 1–{1}", self.day, max_day)
                }
                #[cfg(not(feature = "std"))]
                {
                    "Invalid day".to_string()
                }
            };
            return Err(CalendarError::InvalidParameters(msg));
        }
        Ok(())
    }
}

impl CalendarMetadata for HijriDate {
    fn epoch() -> JDN {
        HIJRI_EPOCH_JDN
    }

    fn cycle_length() -> Option<calendar_core::CycleYear> {
        // 30-year cycle with 11 leap years = 10631 days total
        // 10631 / 30 ≈ 354.37 days/year average
        // Return the cycle length as the cycle unit
        Some(30)
    }

    fn description() -> &'static str {
        "Islamic lunar calendar with 12 months per year and 30-year leap cycles"
    }

    fn cultural_origin() -> &'static str {
        "Islamic calendar, used throughout the Muslim world; epoch based on Hijra in 622 CE"
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hijri_epoch() {
        // 1 Muharram 1 AH should be JDN 1948439
        let hijri = HijriDate {
            year: 1,
            month: 1,
            day: 1,
        };
        assert_eq!(hijri.to_jdn(), 1_948_439);

        // Verify reverse conversion
        let from_jdn = HijriDate::from_jdn(1_948_439).unwrap();
        assert_eq!(from_jdn, hijri);
    }

    #[test]
    fn test_hijri_anchor_1043() {
        // 1 Muharram 1043 AH = JDN 2317689 (July 7, 1633 CE Gregorian)
        // D-R astronomical tabular variant (Thursday epoch).
        // Sultan Agung reform anchor: civil variant is 1 day later (JDN 2317690).
        let hijri = HijriDate {
            year: 1043,
            month: 1,
            day: 1,
        };
        let jdn = hijri.to_jdn();
        assert_eq!(jdn, 2_317_689, "1 Muharram 1043 AH should be JDN 2317689");

        // Verify reverse conversion
        let from_jdn = HijriDate::from_jdn(2_317_689).unwrap();
        assert_eq!(from_jdn, hijri);
    }

    #[test]
    fn test_hijri_anchor_1355() {
        // 1 Muharram 1355 AH = JDN 2428251 (March 23, 1936 CE Gregorian)
        // D-R astronomical tabular variant.
        let hijri = HijriDate {
            year: 1355,
            month: 1,
            day: 1,
        };
        assert_eq!(hijri.to_jdn(), 2_428_251);

        let from_jdn = HijriDate::from_jdn(2_428_251).unwrap();
        assert_eq!(from_jdn, hijri);
    }

    #[test]
    fn test_hijri_anchor_1446() {
        // 1 Muharram 1446 AH = JDN 2460499 (July 7, 2024 CE Gregorian)
        // D-R astronomical tabular variant. Matches Saudi Umm al-Qura announcement
        // of 1 Muharram 1446 AH = July 7, 2024.
        let hijri = HijriDate {
            year: 1446,
            month: 1,
            day: 1,
        };
        assert_eq!(hijri.to_jdn(), 2_460_499);

        let from_jdn = HijriDate::from_jdn(2_460_499).unwrap();
        assert_eq!(from_jdn, hijri);
    }

    #[test]
    fn test_leap_year_logic() {
        // Leap years in a 30-year cycle: 2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29
        let leap_positions = vec![2, 5, 7, 10, 13, 16, 18, 21, 24, 26, 29];

        for year_in_cycle in 1..=30 {
            let is_leap = is_leap_year(year_in_cycle);
            let expected = leap_positions.contains(&year_in_cycle);
            assert_eq!(
                is_leap, expected,
                "Year {year_in_cycle} leap status mismatch"
            );
        }

        // Also test years in the next cycle
        for year_in_cycle in 1..=30 {
            let year = 30 + year_in_cycle;
            let is_leap = is_leap_year(year);
            let expected = leap_positions.contains(&year_in_cycle);
            assert_eq!(is_leap, expected, "Year {year} leap status mismatch");
        }
    }

    #[test]
    fn test_month_days() {
        // Odd-numbered months have 30 days, even have 29
        // Except month 12 which has 29 in common years, 30 in leap years
        let common_year = 1;
        for month in 1..=12 {
            let days = days_in_month(month, common_year);
            if month % 2 == 1 {
                assert_eq!(days, 30, "Month {month} should have 30 days");
            } else if month == 12 {
                assert_eq!(days, 29, "Month 12 should have 29 days in common year");
            } else {
                assert_eq!(days, 29, "Month {month} should have 29 days");
            }
        }

        let leap_year = 2; // Year 2 is leap
        assert_eq!(
            days_in_month(12, leap_year),
            30,
            "Month 12 should have 30 days in leap year"
        );
    }

    #[test]
    fn test_round_trip_conversions() {
        // Test round-trip for various dates across the supported range
        let test_cases = vec![
            (1, 1, 1),      // Epoch
            (1, 12, 29),    // End of first year
            (2, 1, 1),      // Second year (leap year)
            (100, 6, 15),   // Mid-range
            (500, 8, 22),   // Mid-range
            (1043, 1, 1),   // Sultan Agung epoch
            (1355, 1, 1),   // Historical anchor
            (1446, 1, 1),   // Modern anchor
            (1500, 5, 10),  // Recent
            (1600, 12, 30), // Near max
        ];

        for (year, month, day) in test_cases {
            let hijri = HijriDate { year, month, day };
            let jdn = hijri.to_jdn();
            let back = HijriDate::from_jdn(jdn).unwrap();
            assert_eq!(hijri, back, "Round-trip failed for {year}-{month}-{day}");
        }
    }
}
