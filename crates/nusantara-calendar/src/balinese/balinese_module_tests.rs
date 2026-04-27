//! Tests for the nusantara-calendar balinese module

#[cfg(test)]
mod tests {
    use crate::balinese::{BalineseDate, OfficialBalineseDate};
    use calendar_core::{CalendarDate, CalendarMetadata, HasAuspiciousness};

    #[test]
    fn test_balinese_date_creation() {
        let date = BalineseDate::from_ymd(2026, 3, 19).unwrap();
        assert_eq!(date.gregorian_year, 2026);
        assert_eq!(date.gregorian_month, 3);
        assert_eq!(date.gregorian_day, 19);
    }

    #[test]
    fn test_calendar_date_trait() {
        let date = BalineseDate::from_ymd(2026, 3, 19).unwrap();

        // Test calendar name
        assert_eq!(BalineseDate::calendar_name(), "Balinese Saka Calendar");

        // Test JDN conversion
        let jdn = date.to_jdn();
        let date2 = BalineseDate::from_jdn(jdn).unwrap();
        assert_eq!(date, date2);
    }

    #[test]
    fn test_calendar_metadata_trait() {
        assert_eq!(BalineseDate::epoch(), 1_749_630); // Saka epoch: proleptic Gregorian 78-03-22
        assert_eq!(BalineseDate::cycle_length(), None); // No year-level cycle
        assert!(!BalineseDate::description().is_empty());
        assert!(!BalineseDate::cultural_origin().is_empty());
    }

    #[test]
    fn test_auspiciousness_trait_panics() {
        let date = BalineseDate::from_ymd(2026, 3, 19).unwrap();

        // auspiciousness_for is not yet implemented — must panic
        let date_clone = date.clone();
        let result = std::panic::catch_unwind(move || {
            date_clone.auspiciousness_for(&calendar_core::Activity::Marriage)
        });
        assert!(
            result.is_err(),
            "auspiciousness_for should panic (unimplemented)"
        );

        // is_auspicious_day is not yet implemented — must panic
        let result = std::panic::catch_unwind(move || date.is_auspicious_day());
        assert!(
            result.is_err(),
            "is_auspicious_day should panic (unimplemented)"
        );
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
    }

    #[test]
    fn test_balinese_string_format() {
        let date = BalineseDate::from_ymd(2026, 3, 22).unwrap();
        let balinese_str = date.to_balinese_string();

        // Should contain Saka year
        assert!(balinese_str.contains("Saka"));
        // Should contain day names
        assert!(!balinese_str.is_empty());
    }

    #[test]
    fn test_round_trip_conversions() {
        // Test several dates
        let test_dates = [
            (2026, 3, 22),  // Saka new year
            (2026, 1, 1),   // Gregorian new year
            (2025, 12, 31), // Year end
            (2000, 6, 15),  // Y2K era
            (1800, 1, 1),   // Minimum year
            (2200, 12, 31), // Maximum year
        ];

        for (year, month, day) in test_dates {
            let original = BalineseDate::from_ymd(year, month, day).unwrap();
            let jdn = original.to_jdn();
            let round_trip = BalineseDate::from_jdn(jdn).unwrap();
            assert_eq!(
                original, round_trip,
                "Round-trip failed for {year}-{month}-{day}",
            );
        }
    }

    #[test]
    fn test_official_crate_access() {
        let date = BalineseDate::from_ymd(2026, 3, 19).unwrap();
        let official = date.as_official();

        // Test that we can access official crate fields
        assert_eq!(official.gregorian_year, 2026);
        assert_eq!(official.gregorian_month, 3);
        assert_eq!(official.gregorian_day, 19);
    }

    #[test]
    fn test_from_official_conversion() {
        let official_date = OfficialBalineseDate::from_ymd(2026, 3, 19).unwrap();
        let wrapper_date = BalineseDate::from_official(official_date);

        assert_eq!(wrapper_date.gregorian_year, 2026);
        assert_eq!(wrapper_date.gregorian_month, 3);
        assert_eq!(wrapper_date.gregorian_day, 19);
    }
}
