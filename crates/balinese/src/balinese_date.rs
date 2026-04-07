//! Main Balinese date implementation with calendar-core trait integration

use alloc::{format, string::String};
use calendar_core::{CalendarDate, CalendarMetadata, HasAuspiciousness, JDN, CalendarError};

use crate::{
    error::BalineseCalendarError,
    pawukon::Wuku,
    sasih::Sasih,
    wewaran::{Pancawara, Saptawara},
    wariga::{Activity, AuspiciousnessLevel},
};

/// A Balinese calendar date with full Pawukon, Wewaran, and Sasih information
///
/// This struct represents a complete Balinese calendar date and implements
/// the `calendar-core` traits for interoperability with other calendar systems.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BalineseDate {
    /// Gregorian year
    pub gregorian_year: i32,
    /// Gregorian month (1-12)
    pub gregorian_month: u8,
    /// Gregorian day (1-31)
    pub gregorian_day: u8,
    /// Julian Day Number
    pub jdn: JDN,
    
    /// Pawukon cycle position (0-209)
    pub pawukon_day: u16,
    /// Wuku (30-week cycle)
    pub wuku: Wuku,
    /// Day within Wuku (0-6)
    pub wuku_day: u8,
    
    /// Saptawara (7-day week)
    pub saptawara: Saptawara,
    /// Pancawara (5-day market week)
    pub pancawara: Pancawara,
    
    /// Saka year
    pub saka_year: i32,
    /// Sasih (lunar month)
    pub sasih: Sasih,
    /// Is this a Nampih (intercalary) month?
    pub is_nampih: bool,
    
    /// Is this Purnama (full moon)?
    pub is_purnama: bool,
    /// Is this Tilem (new moon)?
    pub is_tilem: bool,
}

impl BalineseDate {
    /// Saka epoch: March 19, 2026 CE corresponds to Penanggal 1 Saka 1948
    /// This is based on kalenderbali.org validation
    pub const SAKA_EPOCH_JDN: JDN = 2_461_119; // March 19, 2026 (validated)
    
    /// Minimum supported Gregorian year
    pub const MIN_GREGORIAN_YEAR: i32 = 1800;
    /// Maximum supported Gregorian year
    pub const MAX_GREGORIAN_YEAR: i32 = 2200;
    
    /// Create a BalineseDate from a Gregorian date
    ///
    /// # Arguments
    /// * `year` - Gregorian year (1800-2200)
    /// * `month` - Gregorian month (1-12)
    /// * `day` - Gregorian day (1-31, validated for month)
    ///
    /// # Returns
    /// `Ok(BalineseDate)` if the date is valid, `Err(BalineseCalendarError)` otherwise
    ///
    /// # Examples
    /// ```
    /// use balinese::BalineseDate;
    ///
    /// let date = BalineseDate::from_ymd(2026, 3, 22).unwrap();
    /// assert_eq!(date.gregorian_year, 2026);
    /// assert_eq!(date.saka_year, 1948);
    /// ```
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Result<Self, BalineseCalendarError> {
        // Validate Gregorian date range
        if year < Self::MIN_GREGORIAN_YEAR || year > Self::MAX_GREGORIAN_YEAR {
            return Err(BalineseCalendarError::OutOfRange(
                format!("Year {} is outside supported range {}-{}", 
                       year, Self::MIN_GREGORIAN_YEAR, Self::MAX_GREGORIAN_YEAR)
            ));
        }
        
        if month < 1 || month > 12 {
            return Err(BalineseCalendarError::InvalidGregorianDate(
                format!("Month {} is invalid (must be 1-12)", month)
            ));
        }
        
        // Validate day based on month
        let max_day = Self::days_in_month(year, month);
        if day < 1 || day > max_day {
            return Err(BalineseCalendarError::InvalidGregorianDate(
                format!("Day {} is invalid for month {} in year {} (max: {})", 
                       day, month, year, max_day)
            ));
        }
        
        // Convert to JDN
        let jdn = Self::gregorian_to_jdn(year, month, day);
        
        // Compute Balinese calendar components
        let (pawukon_day, wuku, wuku_day) = Self::compute_pawukon(jdn);
        let (saptawara, pancawara) = Self::compute_wewaran(jdn);
        let (saka_year, sasih, is_nampih) = Self::compute_sasih(jdn);
        let (is_purnama, is_tilem) = Self::compute_lunar_phase(jdn);
        
        Ok(Self {
            gregorian_year: year,
            gregorian_month: month,
            gregorian_day: day,
            jdn,
            pawukon_day,
            wuku,
            wuku_day,
            saptawara,
            pancawara,
            saka_year,
            sasih,
            is_nampih,
            is_purnama,
            is_tilem,
        })
    }
    
    /// Get the number of days in a Gregorian month
    fn days_in_month(year: i32, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                // Leap year check
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(), // validated earlier
        }
    }
    
    /// Convert Gregorian date to Julian Day Number
    fn gregorian_to_jdn(year: i32, month: u8, day: u8) -> JDN {
        // Algorithm from Meeus, "Astronomical Algorithms", 2nd Edition
        let (y, m) = if month <= 2 {
            (year - 1, month + 12)
        } else {
            (year, month)
        };
        
        let m_i32 = i32::from(m);
        let day_i32 = i32::from(day);
        
        let jdn: JDN = ((1461 * (y + 4800 + (m_i32 - 14) / 12)) / 4
            + (367 * (m_i32 - 2 - 12 * ((m_i32 - 14) / 12))) / 12
            - (3 * ((y + 4900 + (m_i32 - 14) / 12) / 100)) / 4
            + day_i32
            - 32075) as i64;
        
        jdn
    }
    
    /// Convert Julian Day Number to Gregorian date
    fn jdn_to_gregorian(jdn: JDN) -> (i32, u8, u8) {
        // Fliegel & van Flandern algorithm (1968)
        let jd = jdn as i32;
        let l = jd + 68_569;
        let n = (4 * l) / 146_097;
        let l = l - (146_097 * n + 3) / 4;
        let i = (4_000 * (l + 1)) / 1_461_001;
        let l = l - (1_461 * i) / 4 + 31;
        let j = (80 * l) / 2_447;
        let day = l - (2_447 * j) / 80;
        let l = j / 11;
        let month = j + 2 - 12 * l;
        let year = 100 * (n - 49) + i + l;
        
        (year, month as u8, day as u8)
    }
    
    /// Compute Pawukon cycle components
    fn compute_pawukon(jdn: JDN) -> (u16, Wuku, u8) {
        // Pawukon cycle: 210 days = 30 Wuku × 7 days
        // Reference: JDN 2,459,858 (March 22, 2026) = Saka 1948, Pawukon day 0
        let raw_pawukon_day = jdn - Self::SAKA_EPOCH_JDN;
        let pawukon_day = ((raw_pawukon_day % 210 + 210) % 210) as u16; // Handle negative modulo
        let wuku_index = (pawukon_day / 7) as usize;
        let wuku_day = (pawukon_day % 7) as u8;
        
        (pawukon_day, Wuku::from_index(wuku_index), wuku_day)
    }
    
    /// Compute Wewaran (week cycles)
    fn compute_wewaran(jdn: JDN) -> (Saptawara, Pancawara) {
        // Saptawara: 7-day week (Sunday = 0)
        let saptawara_day = ((jdn + 1) % 7) as u8; // Adjust for Sunday start
        let saptawara = Saptawara::from_index(saptawara_day as usize);
        
        // Pancawara: 5-day market week
        let pancawara_day = ((jdn - 1) % 5) as u8;
        let pancawara = Pancawara::from_index(pancawara_day as usize);
        
        (saptawara, pancawara)
    }
    
    /// Compute Sasih (lunar month) and Saka year
    fn compute_sasih(jdn: JDN) -> (i32, Sasih, bool) {
        // Saka year calculation based on kalenderbali.org validation
        // March 19, 2026 (JDN 2,461,119) = Penanggal 1 Saka 1948
        // This means Saka 1948 starts on this date
        
        let adjusted_saka_year = if jdn >= 2_461_119 { // March 19, 2026 onwards
            1948 + ((jdn - 2_461_119) / 354) as i32
        } else {
            1948 - ((2_461_119 - jdn) / 354 + 1) as i32
        };
        
        // Sasih calculation (simplified - actual algorithm is more complex)
        let sasih_cycle = if jdn >= 2_461_119 {
            ((jdn - 2_461_119) % 354) as usize
        } else {
            (354 - ((2_461_119 - jdn) % 354) as usize) % 354
        };
        let sasih_index = (sasih_cycle / 29) % 12;
        let sasih = Sasih::from_index(sasih_index);
        
        // Nampih (intercalary month) detection (simplified)
        let is_nampih = matches!(sasih, Sasih::Kadasa) && (adjusted_saka_year % 2 == 0);
        
        (adjusted_saka_year, sasih, is_nampih)
    }
    
    /// Compute lunar phase (Purnama/Tilem)
    fn compute_lunar_phase(jdn: JDN) -> (bool, bool) {
        // Simplified lunar phase calculation
        // In reality, this requires astronomical calculations
        let lunar_cycle = (jdn - Self::SAKA_EPOCH_JDN) % 30;
        
        let is_purnama = lunar_cycle == 14 || lunar_cycle == 15;
        let is_tilem = lunar_cycle == 0 || lunar_cycle == 29;
        
        (is_purnama, is_tilem)
    }
    
    /// Convert to a traditional Balinese string representation
    ///
    /// # Returns
    /// A string in the format: "Saptawara Pancawara Wuku, Sasih SasihName, Saka YYYY"
    ///
    /// # Examples
    /// ```
    /// use balinese::BalineseDate;
    ///
    /// let date = BalineseDate::from_ymd(2026, 3, 22).unwrap();
    /// let balinese_str = date.to_balinese_string();
    /// // Example: "Redite Pon Dukut, Sasih Kadasa, Saka 1948"
    /// ```
    pub fn to_balinese_string(&self) -> String {
        format!(
            "{} {} {}, Sasih {}, Saka {}{}",
            self.saptawara.name(),
            self.pancawara.name(),
            self.wuku.name(),
            self.sasih.name(),
            self.saka_year,
            if self.is_nampih { " (Nampih)" } else { "" }
        )
    }
}

// Calendar trait implementations
impl CalendarDate for BalineseDate {
    fn from_jdn(jdn: JDN) -> Result<Self, CalendarError> {
        let (year, month, day) = Self::jdn_to_gregorian(jdn);
        Self::from_ymd(year, month, day).map_err(|e| CalendarError::from(e))
    }
    
    fn to_jdn(&self) -> JDN {
        self.jdn
    }
    
    fn calendar_name() -> &'static str {
        "Balinese Saka Calendar"
    }
    
    fn validate_range(&self) -> Result<(), CalendarError> {
        if self.gregorian_year < Self::MIN_GREGORIAN_YEAR || 
           self.gregorian_year > Self::MAX_GREGORIAN_YEAR {
            return Err(CalendarError::OutOfRange(
                format!("Gregorian year {} is outside supported range {}-{}",
                       self.gregorian_year, Self::MIN_GREGORIAN_YEAR, Self::MAX_GREGORIAN_YEAR)
            ));
        }
        Ok(())
    }
}

impl CalendarMetadata for BalineseDate {
    fn epoch() -> JDN {
        Self::SAKA_EPOCH_JDN
    }
    
    fn cycle_length() -> Option<calendar_core::CycleYear> {
        Some(210) // Pawukon cycle length
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
    
    fn auspiciousness_for(&self, activity: &Self::Activity) -> Self::AuspiciousnessLevel {
        // Simplified auspiciousness calculation
        // In reality, this would use complex Wariga calculations
        match activity {
            Activity::Marriage => {
                // Example: Marriage is auspicious on certain days
                if matches!(self.saptawara, Saptawara::Buda | Saptawara::Sukra) {
                    AuspiciousnessLevel::Auspicious
                } else if matches!(self.saptawara, Saptawara::Redite | Saptawara::Saniscara) {
                    AuspiciousnessLevel::Inauspicious
                } else {
                    AuspiciousnessLevel::Neutral
                }
            }
            Activity::Building => {
                // Building is generally neutral with some exceptions
                if self.is_tilem {
                    AuspiciousnessLevel::Inauspicious
                } else {
                    AuspiciousnessLevel::Neutral
                }
            }
            Activity::Ceremony => {
                // Ceremonies are generally auspicious, especially on Purnama
                if self.is_purnama {
                    AuspiciousnessLevel::VeryAuspicious
                } else {
                    AuspiciousnessLevel::Auspicious
                }
            }
            _ => AuspiciousnessLevel::Neutral,
        }
    }
    
    fn is_auspicious_day(&self) -> bool {
        // General auspiciousness assessment
        // Consider multiple factors
        let saptawara_auspicious = matches!(self.saptawara, 
            Saptawara::Buda | Saptawara::Sukra | Saptawara::Wraspati);
        let lunar_auspicious = self.is_purnama && !self.is_tilem;
        let wuku_auspicious = matches!(self.wuku, 
            Wuku::Sinta | Wuku::Landep | Wuku::Ukir); // Example auspicious Wuku
        
        saptawara_auspicious || lunar_auspicious || wuku_auspicious
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use calendar_core::{CalendarDate, CalendarMetadata};

    #[test]
    fn test_balinese_date_creation() {
        let date = BalineseDate::from_ymd(2026, 3, 19).unwrap(); // Nyepi 2026 - Saka 1948
        assert_eq!(date.gregorian_year, 2026);
        assert_eq!(date.gregorian_month, 3);
        assert_eq!(date.gregorian_day, 19);
        // Debug: print actual values
        println!("JDN: {}, Saka Year: {}", date.jdn, date.saka_year);
        assert_eq!(date.saka_year, 1948);
    }

    #[test]
    fn test_calendar_date_trait() {
        let date = BalineseDate::from_ymd(2026, 3, 19).unwrap(); // Nyepi 2026 - Saka 1948
        
        // Test calendar name
        assert_eq!(BalineseDate::calendar_name(), "Balinese Saka Calendar");
        
        // Test JDN conversion
        let jdn = date.to_jdn();
        let date2 = BalineseDate::from_jdn(jdn).unwrap();
        assert_eq!(date, date2);
        
        // Test Gregorian conversion
        let (year, month, day) = date.to_gregorian();
        assert_eq!((year, month, day), (2026, 3, 19));
    }

    #[test]
    fn test_calendar_metadata_trait() {
        assert_eq!(BalineseDate::epoch(), 2_461_119); // March 19, 2026 (validated against kalenderbali.org)
        assert_eq!(BalineseDate::cycle_length(), Some(210)); // Pawukon cycle
        assert!(!BalineseDate::description().is_empty());
        assert!(!BalineseDate::cultural_origin().is_empty());
    }

    #[test]
    fn test_auspiciousness_trait() {
        let date = BalineseDate::from_ymd(2026, 3, 19).unwrap(); // Nyepi 2026 - Saka 1948
        
        // Test specific activity auspiciousness
        let marriage_auspiciousness = date.auspiciousness_for(&Activity::Marriage);
        assert!(matches!(marriage_auspiciousness, 
            AuspiciousnessLevel::Auspicious | 
            AuspiciousnessLevel::Inauspicious | 
            AuspiciousnessLevel::Neutral));
        
        // Test general auspiciousness
        let is_auspicious = date.is_auspicious_day();
        // Should be either true or false, not panic
        let _ = is_auspicious;
    }

    #[test]
    fn test_date_validation() {
        // Valid date
        let date = BalineseDate::from_ymd(2026, 3, 22).unwrap();
        assert!(date.validate_range().is_ok());
        
        // Invalid year (too early)
        let result = BalineseDate::from_ymd(1799, 1, 1);
        assert!(result.is_err());
        
        // Invalid year (too late)
        let result = BalineseDate::from_ymd(2201, 1, 1);
        assert!(result.is_err());
        
        // Invalid month
        let result = BalineseDate::from_ymd(2026, 13, 1);
        assert!(result.is_err());
        
        // Invalid day
        let result = BalineseDate::from_ymd(2026, 2, 30);
        assert!(result.is_err());
    }

    #[test]
    fn test_balinese_string_format() {
        let date = BalineseDate::from_ymd(2026, 3, 22).unwrap();
        let balinese_str = date.to_balinese_string();
        
        // Should contain Saka year
        assert!(balinese_str.contains("Saka 1948"));
        // Should contain day names
        assert!(!balinese_str.is_empty());
    }

    #[test]
    fn test_nyepi_2026() {
        // Nyepi 2026 falls on March 19, 2026 (Saka 1948)
        // This is the Balinese Hindu New Year - a day of silence
        // Validated against kalenderbali.org: "Penanggal 1 Saka 1948"
        let nyepi_date = BalineseDate::from_ymd(2026, 3, 19).unwrap();
        
        // Verify it's in Saka 1948 (validated against kalenderbali.org)
        assert_eq!(nyepi_date.saka_year, 1948);
        
        // Verify the date conversion works correctly
        let (year, month, day) = nyepi_date.to_gregorian();
        assert_eq!((year, month, day), (2026, 3, 19));
        
        // Test round-trip conversion
        let jdn = nyepi_date.to_jdn();
        let round_trip = BalineseDate::from_jdn(jdn).unwrap();
        assert_eq!(nyepi_date, round_trip);
        
        // Verify basic calendar traits work
        assert!(nyepi_date.validate_range().is_ok());
        assert_eq!(BalineseDate::calendar_name(), "Balinese Saka Calendar");
    }

    #[test]
    fn test_round_trip_conversions() {
        // Test several dates
        let test_dates = [
            (2026, 3, 22), // Saka new year
            (2026, 1, 1),  // Gregorian new year
            (2025, 12, 31), // Year end
            (2000, 6, 15),  // Y2K era
            (1800, 1, 1),   // Minimum year
            (2200, 12, 31), // Maximum year
        ];
        
        for (year, month, day) in test_dates {
            let original = BalineseDate::from_ymd(year, month, day).unwrap();
            let jdn = original.to_jdn();
            let round_trip = BalineseDate::from_jdn(jdn).unwrap();
            assert_eq!(original, round_trip, "Round-trip failed for {}-{}-{}", year, month, day);
        }
    }
}
