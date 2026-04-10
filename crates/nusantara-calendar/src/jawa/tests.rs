//! Javanese calendar module tests

use super::*;

// ============================================================================
// CONSTANTS VALIDATION
// ============================================================================

#[test]
fn sultan_agung_epoch_matches_expected() {
    assert_eq!(SULTAN_AGUNG_EPOCH_JDN, 2_317_690);
}

#[test]
fn kurup_asapon_boundaries() {
    assert_eq!(KURUP_ASAPON_START_JDN, 2_428_475);
    assert_eq!(KURUP_ASAPON_END_JDN, 2_475_069);
    assert_eq!(KURUP_ASAPON.start_aj, 1867);
    assert_eq!(KURUP_ASAPON.end_aj, 1986);
}

#[test]
fn supported_year_range() {
    assert_eq!(AJ_MIN, 1555);
    assert_eq!(AJ_MAX, 2474);
    assert_eq!(JDN_MIN, SULTAN_AGUNG_EPOCH_JDN);
    assert_eq!(JDN_MAX, 2_766_190);
}

// ============================================================================
// WINDU YEAR ENUM
// ============================================================================

#[test]
fn windu_year_leap_years() {
    assert!(!WinduYear::Alip.is_leap());
    assert!(!WinduYear::Ehe.is_leap());
    assert!(WinduYear::Jimawal.is_leap());
    assert!(!WinduYear::Je.is_leap());
    assert!(WinduYear::Dal.is_leap());
    assert!(!WinduYear::Be.is_leap());
    assert!(!WinduYear::Wawu.is_leap());
    assert!(WinduYear::Jimakir.is_leap());
}

#[test]
fn windu_year_days_in_year() {
    assert_eq!(WinduYear::Alip.days_in_year(), 354);
    assert_eq!(WinduYear::Jimawal.days_in_year(), 355);
    assert_eq!(WinduYear::Dal.days_in_year(), 355);
    assert_eq!(WinduYear::Jimakir.days_in_year(), 355);
}

#[test]
fn windu_year_from_aj() {
    // Windu cycle: (aj - 1) % 8 maps to year names
    // 0=Alip, 1=Ehe, 2=Jimawal, 3=Je, 4=Dal, 5=Be, 6=Wawu, 7=Jimakir

    // Verify the mapping directly
    assert_eq!(WinduYear::from_aj(1), Some(WinduYear::Alip)); // (1-1)%8 = 0
    assert_eq!(WinduYear::from_aj(2), Some(WinduYear::Ehe)); // (2-1)%8 = 1
    assert_eq!(WinduYear::from_aj(3), Some(WinduYear::Jimawal)); // (3-1)%8 = 2
    assert_eq!(WinduYear::from_aj(8), Some(WinduYear::Jimakir)); // (8-1)%8 = 7
    assert_eq!(WinduYear::from_aj(9), Some(WinduYear::Alip)); // (9-1)%8 = 0, cycle repeats

    // AJ 1959 = Wawu (current as of March 2026 per SPEC.md)
    // (1959-1)%8 = 1958%8 = 6 → Wawu ✓
    assert_eq!(WinduYear::from_aj(1959), Some(WinduYear::Wawu));

    // SPEC.md anchor: AJ 1555 maps to specific windu year
    // (1555-1)%8 = 1554%8 = 2 → Jimawal
    assert_eq!(WinduYear::from_aj(1555), Some(WinduYear::Jimawal));
}

// ============================================================================
// WUKU POSITION
// ============================================================================

#[test]
fn wuku_pos_pawukon_day_roundtrip() {
    for day in 0..210u16 {
        let pos = WukuPos::from_pawukon_day(day).unwrap();
        assert_eq!(pos.pawukon_day(), day);
    }
}

#[test]
fn wuku_pos_out_of_range() {
    assert!(WukuPos::from_pawukon_day(210).is_none());
    assert!(WukuPos::from_pawukon_day(300).is_none());
}

#[test]
fn wuku_names_count() {
    assert_eq!(WUKU_NAMES.len(), 30);
}

// ============================================================================
// PASARAN & SAPTAWARA
// ============================================================================

#[test]
fn pasaran_names_count() {
    assert_eq!(PASARAN_NAMES_NGOKO.len(), 5);
    assert_eq!(PASARAN_NAMES_KRAMA.len(), 5);
    assert_eq!(PASARAN_NEPTU.len(), 5);
}

#[test]
fn saptawara_names_count() {
    assert_eq!(SAPTAWARA_NAMES.len(), 7);
    assert_eq!(SAPTAWARA_NEPTU.len(), 7);
    assert_eq!(SAPTAWARA_INDONESIAN.len(), 7);
}

#[test]
fn wetonan_neptu_calculation() {
    // Selasa (3) + Pon (2) = 5
    assert_eq!(wetonan_neptu(1, 2), 3 + 7); // SAPTAWARA_NEPTU[1] + PASARAN_NEPTU[2]
}

// ============================================================================
// PRANATA MASA
// ============================================================================

#[test]
fn pranata_masa_names_count() {
    assert_eq!(PRANATA_MASA_NAMES.len(), 12);
    assert_eq!(PRANATA_MASA_SOLAR_OFFSETS.len(), 12);
}

// ============================================================================
// KURUP CONSTANTS
// ============================================================================

#[test]
fn kurup_length_constants() {
    assert_eq!(KURUP_LENGTH_YEARS, 120);
    assert_eq!(KURUP_LENGTH_WINDU, 15);
    assert_eq!(KURUP_LENGTH_DAYS, 42_525);
}

#[test]
fn windu_cycle_length() {
    assert_eq!(WINDU_LENGTH_YEARS, 8);
    assert_eq!(WINDU_LENGTH_DAYS, 2835);
    // Verify: 5 common years (354) + 3 leap years (355) = 2835
    assert_eq!(5 * 354 + 3 * 355, 2835);
}

// ============================================================================
// DINA MULYA
// ============================================================================

#[test]
fn dina_mulya_entries() {
    assert!(!DINA_MULYA.is_empty());
    // Each entry should have valid wuku and saptawara
    for entry in DINA_MULYA {
        assert!(entry.wuku < 30, "Wuku index out of range");
        assert!(entry.saptawara < 7, "Saptawara index out of range");
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS (PLACEHOLDER)
// ============================================================================

#[test]
fn calendar_metadata() {
    assert_eq!(JavaneseDay::epoch(), SULTAN_AGUNG_EPOCH_JDN);
    assert_eq!(
        JavaneseDay::calendar_name(),
        "Javanese Calendar (Sultan Agung Era)"
    );
    assert!(!JavaneseDay::description().is_empty());
    assert!(!JavaneseDay::cultural_origin().is_empty());
}

#[test]
fn javanese_day_out_of_range() {
    // Below minimum
    assert!(JavaneseDay::from_jdn(JDN_MIN - 1).is_none());
    // Above maximum
    assert!(JavaneseDay::from_jdn(JDN_MAX + 1).is_none());
}

// ============================================================================
// KNOWN ANCHORS (to be implemented)
// ============================================================================

// TODO: Implement these when JDN → Javanese conversion is complete:
// - Known anchor: JDN 2317690 → 1 Sura 1555 AJ, Jumat Legi, wuku Sinta pos 1
// - Known anchor: 1945-08-17 (Proklamasi) → Jumat Legi (verify historical record)
// - Kurup boundary: 1936-03-24 → Selasa Pon, Alip year
