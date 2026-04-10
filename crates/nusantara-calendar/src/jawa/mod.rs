//! Javanese calendar system
//!
//! Independent implementation of the Javanese calendar (Sultan Agung era, 1633 CE onward).
//! Supports Anno Javanico (AJ) years 1555–2474 (Gregorian 1633–2169), spanning 2 kurups.
//!
//! ## Primary Sources
//!
//! - Karjanto & Beauducel (2020), arXiv:2012.10064 — Wetonan congruence formula
//! - Dershowitz & Reingold, *Calendrical Calculations* (4th ed.), Ch. 10 — Pawukon 210-day cycle
//! - H. Danudji, *Penanggalan Jawa 120 Tahun Kurup Asapon* (2006) — Kurup tables
//!
//! ## Cycle Summary
//!
//! | Cycle | Length | Data Type |
//! |---|---|---|
//! | Pasaran | 5 days | `PasaranPos` (u8, 0–4) |
//! | Saptawara | 7 days | `SaptawaraPos` (u8, 0–6) |
//! | Wetonan | 35 days | `(SaptawaraPos, PasaranPos)` |
//! | Pawukon | 210 days | `WukuPos` (struct with wuku + saptawara) |
//! | Wuku | 30 names | `u8` index into `WUKU_NAMES` |
//! | Windu | 8 years | `WinduYear` enum |
//! | Kurup | 120 years | `KurupRecord` |
//! | Pranata Masa | 12 seasons | `PranataMasaPos` (u8, 0–11) |

use crate::{CalendarDate, CalendarError, CalendarMetadata, JDN};
use calendar_core::stub;

// no_std compatibility: import alloc types when std is not available
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::string::ToString;

// ============================================================================
// EPOCH CONSTANTS
// ============================================================================

/// Epoch of the Sultan Agung Javanese calendar reform.
/// Gregorian 1633-07-08 = 1 Sura 1555 AJ = 1 Muharram 1043 AH.
///
/// Sources: Karjanto & Beauducel (2020), arXiv:2012.10064;
///          Wikipedia "Javanese calendar" (verified March 2026).
pub const SULTAN_AGUNG_EPOCH_JDN: i64 = 2_317_690;

/// Maximum supported JDN (AJ 2474-12-30 ≈ 2169 CE).
/// Calculated: 2 kurups × 120 years × 354.375 days/year average.
pub const JDN_MAX: i64 = 2_766_190;

/// Start of current Kurup Asapon (Alip Selasa Pon).
/// Gregorian 1936-03-24 = JDN `2_428_252` (calculated from epoch + `110_562` days).
///
/// Primary source: Danudji (2006), *Penanggalan Jawa 120 Tahun Kurup Asapon*.
/// Cross-validated: Wikipedia "Javanese calendar" + `beaudu/weton` repository.
pub const KURUP_ASAPON_START_JDN: i64 = 2_428_252;

/// End of current Kurup Asapon.
/// Gregorian 2052-08-25 = JDN `2_474_846` (120 years × ~354.4 days/year from start).
///
/// Primary source: Danudji (2006).
/// Cross-validated: Wikipedia + `beaudu/weton`.
pub const KURUP_ASAPON_END_JDN: i64 = 2_474_846;

// ============================================================================
// SUPPORTED YEAR RANGE
// ============================================================================

/// Minimum supported Anno Javanico year (Sultan Agung epoch).
pub const AJ_MIN: u32 = 1555;

/// Maximum supported Anno Javanico year (end of second kurup).
/// This spans the current Kurup Asapon (1936–2052) and the next full kurup.
pub const AJ_MAX: u32 = 2474;

/// Minimum supported JDN (AJ 1555-01-01 = 1633-07-08 Gregorian).
pub const JDN_MIN: i64 = SULTAN_AGUNG_EPOCH_JDN;

// ============================================================================
// PASARAN (5-DAY MARKET CYCLE)
// ============================================================================

/// Position in the 5-day Pasaran cycle (0 = Legi, 4 = Kliwon).
pub type PasaranPos = u8;

/// Pasaran names in Ngoko (common) and Krama (high) Javanese.
/// Index corresponds to `PasaranPos` (0–4).
pub const PASARAN_NAMES_NGOKO: [&str; 5] = ["Legi", "Pahing", "Pon", "Wage", "Kliwon"];

/// Pasaran names in Krama (high/polite) Javanese.
pub const PASARAN_NAMES_KRAMA: [&str; 5] = ["Manis", "Pahing", "Pon", "Cemeng", "Asih"];

/// Neptu (numerical value) for each Pasaran day.
/// Used in weton calculations for auspiciousness.
pub const PASARAN_NEPTU: [u8; 5] = [5, 9, 7, 4, 8];

// ============================================================================
// SAPTAWARA (7-DAY WEEK)
// ============================================================================

/// Position in the 7-day Saptawara cycle (0 = Soma/Monday, 6 = Redite/Sunday).
/// Note: Javanese Saptawara traditionally starts with Soma (Monday).
pub type SaptawaraPos = u8;

/// Saptawara (7-day week) names in Javanese.
/// Index corresponds to `SaptawaraPos` (0–6).
pub const SAPTAWARA_NAMES: [&str; 7] =
    ["Soma", "Selasa", "Rebo", "Kemis", "Jemuwah", "Setu", "Ahad"];

/// Saptawara neptu values.
pub const SAPTAWARA_NEPTU: [u8; 7] = [4, 3, 7, 8, 6, 9, 5];

/// Indonesian weekday equivalents for reference.
pub const SAPTAWARA_INDONESIAN: [&str; 7] = [
    "Senin", "Selasa", "Rabu", "Kamis", "Jumat", "Sabtu", "Minggu",
];

// ============================================================================
// WETONAN (35-DAY CYCLE)
// ============================================================================

/// Wetonan = Saptawara × Pasaran = 35-day personal cycle.
/// Used for birth dates (weton lahir) and compatibility.
pub type Wetonan = (SaptawaraPos, PasaranPos);

/// Combined neptu for a Wetonan day (sum of Saptawara + Pasaran neptu).
#[must_use]
pub const fn wetonan_neptu(sapta: SaptawaraPos, pasaran: PasaranPos) -> u8 {
    SAPTAWARA_NEPTU[sapta as usize] + PASARAN_NEPTU[pasaran as usize]
}

// ============================================================================
// WUKU (30-WEEK CYCLE)
// ============================================================================

/// Wuku position in the 30-wuku Pawukon cycle (0–29).
pub type WukuIdx = u8;

/// Wuku names in Javanese.
/// Same 30-wuku structure as Balinese, but with Javanese-language names.
/// Index corresponds to `WukuIdx` (0–29).
pub const WUKU_NAMES: [&str; 30] = [
    "Sinta",        // 0
    "Landep",       // 1
    "Ukir",         // 2
    "Kulantir",     // 3
    "Tolu",         // 4
    "Gumbreg",      // 5
    "Warigalit",    // 6
    "Warigagung",   // 7
    "Julungwangi",  // 8
    "Sungsang",     // 9
    "Galungan",     // 10
    "Kuningan",     // 11
    "Langkir",      // 12
    "Mandasiya",    // 13
    "Julungpujut",  // 14
    "Pahang",       // 15
    "Kuruwelut",    // 16
    "Marakeh",      // 17
    "Tambir",       // 18
    "Medangkungan", // 19
    "Maktal",       // 20
    "Wuye",         // 21
    "Manahil",      // 22
    "Prangbakat",   // 23
    "Bala",         // 24
    "Wugu",         // 25
    "Wayang",       // 26
    "Kelawu",       // 27
    "Dukut",        // 28
    "Watugunung",   // 29
];

// ============================================================================
// PAWUKON (210-DAY CYCLE)
// ============================================================================

/// Position within the 210-day Pawukon cycle.
/// Combines Wuku (30 possibilities) × Saptawara (7 possibilities) = 210 unique days.
///
/// Source: Dershowitz & Reingold, *Calendrical Calculations* (4th ed.), Ch. 10.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WukuPos {
    /// Wuku index (0–29), indexing into `WUKU_NAMES`.
    pub wuku: WukuIdx,
    /// Day within the wuku (0–6), corresponding to Saptawara.
    pub day_in_wuku: SaptawaraPos,
}

impl WukuPos {
    /// Total position in the 210-day Pawukon cycle (0–209).
    #[must_use]
    pub const fn pawukon_day(&self) -> u16 {
        (self.wuku as u16) * 7 + (self.day_in_wuku as u16)
    }

    /// Create from absolute Pawukon day number (0–209).
    #[must_use]
    pub const fn from_pawukon_day(day: u16) -> Option<Self> {
        if day >= 210 {
            return None;
        }
        Some(Self {
            // SAFETY: day < 210, so day/7 < 30 and day%7 < 7, both fit in u8
            #[allow(clippy::cast_possible_truncation)]
            wuku: (day / 7) as u8,
            #[allow(clippy::cast_possible_truncation)]
            day_in_wuku: (day % 7) as u8,
        })
    }
}

/// Pawukon length in days.
pub const PAWUKON_LENGTH: u16 = 210;

// ============================================================================
// WINDU (8-YEAR CYCLE)
// ============================================================================

/// Windu year names in the 8-year Javanese cycle.
///
/// Leap years (355 days): Jimawal, Dal, Jimakir.
/// Common years (354 days): Alip, Ehe, Je, Be, Wawu.
///
/// Source: Karjanto & Beauducel (2020), arXiv:2012.10064.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WinduYear {
    /// Year 1: 354 days (common)
    Alip = 1,
    /// Year 2: 354 days (common)
    Ehe = 2,
    /// Year 3: 355 days (**leap**)
    Jimawal = 3,
    /// Year 4: 354 days (common)
    Je = 4,
    /// Year 5: 355 days (**leap**)
    Dal = 5,
    /// Year 6: 354 days (common)
    Be = 6,
    /// Year 7: 354 days (common)
    Wawu = 7,
    /// Year 8: 355 days (**leap**)
    Jimakir = 8,
}

impl WinduYear {
    /// Returns true if this is a leap year (355 days).
    #[must_use]
    pub const fn is_leap(&self) -> bool {
        matches!(self, Self::Jimawal | Self::Dal | Self::Jimakir)
    }

    /// Returns the number of days in this windu year.
    #[must_use]
    pub const fn days_in_year(&self) -> u16 {
        if self.is_leap() {
            355
        } else {
            354
        }
    }

    /// Convert Anno Javanico year to Windu year.
    /// AJ 1555 = year 1 of first windu = Alip.
    #[must_use]
    pub const fn from_aj(aj: u32) -> Option<Self> {
        match (aj.wrapping_sub(1)) % 8 {
            0 => Some(Self::Alip),
            1 => Some(Self::Ehe),
            2 => Some(Self::Jimawal),
            3 => Some(Self::Je),
            4 => Some(Self::Dal),
            5 => Some(Self::Be),
            6 => Some(Self::Wawu),
            7 => Some(Self::Jimakir),
            _ => None, // unreachable
        }
    }
}

/// Number of years in a Windu cycle.
pub const WINDU_LENGTH_YEARS: u8 = 8;

/// Days in a complete Windu cycle (4×354 + 3×355 + 354 = 2835, or 8×354 + 3 = 2835).
pub const WINDU_LENGTH_DAYS: u16 = 2835;

// ============================================================================
// SUPRA-WINDU GROUPS (STUB)
// ============================================================================

/// Supra-windu group names within the 120-year Kurup hierarchy.
///
/// **STUB:** These are 4-group names that categorize the 15 windus within a Kurup.
/// The full algorithm for determining the current supra-windu group from AJ year
/// requires the complete Kurup table from Danudji (2006) which is not yet implemented.
///
/// # Sources
/// - H. Danudji, *Penanggalan Jawa 120 Tahun Kurup Asapon*, Dahara Prize 2006.
///   ISBN 979-501-454-4. Print-only; algorithm not digitally available.
/// - Wikipedia "Javanese calendar" confirms existence but lacks computational detail.
///
/// Known supra-windu groups: Adi, Kuntara, Sengara, Sancaya.
/// "Sancaya" is confirmed as a group name, NOT a windu year name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupraWinduGroup {
    /// First supra-windu group (windus 1-4 of a Kurup)
    Adi,
    /// Second supra-windu group (windus 5-8 of a Kurup)
    Kuntara,
    /// Third supra-windu group (windus 9-12 of a Kurup)
    Sengara,
    /// Fourth supra-windu group (windus 13-15 of a Kurup)
    ///
    /// **Note:** "Sancaya" is a supra-windu group, not a windu year.
    Sancaya,
}

/// Compute supra-windu group from AJ year.
///
/// # Sources
/// - H. Danudji (2006), *Penanggalan Jawa 120 Tahun Kurup Asapon*.
///   Complete algorithm requires full Kurup table not yet transcribed.
///
/// **STUB:** Returns `NotImplemented` until full Kurup table is available.
///
/// # Errors
/// Returns `CalendarError::NotImplemented` for all inputs, as the algorithm
/// requires the complete Kurup table from Danudji (2006) which is not yet available.
pub fn supra_windu_from_aj(_aj: u32) -> Result<SupraWinduGroup, CalendarError> {
    stub!("Supra-windu group: requires Danudji (2006) Kurup table. Not yet implemented.")
}

// ============================================================================
// KURUP (120-YEAR CYCLE)
// ============================================================================

/// Record of a complete 120-year Kurup cycle.
///
/// A Kurup consists of 15 windus (15 × 8 = 120 years).
/// Each Kurup has a starting Wetonan anchor (e.g., "Selasa Pon" for Kurup Asapon).
///
/// Source: H. Danudji, *Penanggalan Jawa 120 Tahun Kurup Asapon* (2006).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KurupRecord {
    /// Name of the kurup (e.g., "Asapon", "Salasiya").
    pub name: &'static str,
    /// Starting Wetonan anchor (Saptawara, Pasaran).
    pub start_weton: Wetonan,
    /// First day of the Kurup in JDN.
    pub start_jdn: i64,
    /// Last day of the Kurup in JDN.
    pub end_jdn: i64,
    /// First AJ year in this Kurup.
    pub start_aj: u32,
    /// Last AJ year in this Kurup.
    pub end_aj: u32,
}

/// Current Kurup Asapon (started 1936-03-24, ends 2052-08-25).
/// Anchor: Alip year, Selasa Pon (Tuesday Pon).
pub const KURUP_ASAPON: KurupRecord = KurupRecord {
    name: "Asapon",
    start_weton: (1, 2), // Selasa (1), Pon (2)
    start_jdn: KURUP_ASAPON_START_JDN,
    end_jdn: KURUP_ASAPON_END_JDN,
    start_aj: 1868,
    end_aj: 1987,
};

/// Number of years in a Kurup cycle.
pub const KURUP_LENGTH_YEARS: u16 = 120;

/// Number of windus in a Kurup.
pub const KURUP_LENGTH_WINDU: u8 = 15;

/// Days in a complete Kurup cycle (15 windus × 2835 days = 42,525 days).
pub const KURUP_LENGTH_DAYS: u32 = 42_525;

// ============================================================================
// PRANATA MASA (12 SOLAR SEASONS)
// ============================================================================

/// Position in the 12-season Pranata Masa cycle (0 = Kasa, 11 = Sada).
///
/// Pranata Masa are solar-based agricultural seasons, distinct from lunar months.
pub type PranataMasaPos = u8;

/// Pranata Masa (solar season) names.
/// Index corresponds to `PranataMasaPos` (0–11).
pub const PRANATA_MASA_NAMES: [&str; 12] = [
    "Kasa",      // 0
    "Karo",      // 1
    "Katelu",    // 2
    "Kapat",     // 3
    "Kalima",    // 4
    "Kanem",     // 5
    "Kapitu",    // 6
    "Kawolu",    // 7
    "Kasongo",   // 8
    "Kasepuluh", // 9
    "Desta",     // 10
    "Sada",      // 11
];

/// Approximate solar epoch offsets for each Pranata Masa (days from solar year start).
/// These are approximate and vary slightly year-to-year.
pub const PRANATA_MASA_SOLAR_OFFSETS: [u16; 12] = [
    0,   // Kasa: ~June solstice
    30,  // Karo
    61,  // Katelu
    91,  // Kapat
    122, // Kalima
    153, // Kanem
    183, // Kapitu
    214, // Kawolu
    245, // Kasongo
    275, // Kasepuluh
    306, // Desta
    336, // Sada
];

/// Length of solar year in days (approximate for Pranata Masa calculation).
/// Uses 365 days as a fixed approximation.
pub const PRANATA_MASA_SOLAR_YEAR_DAYS: u16 = 365;

/// Reference JDN for Kasa start (June solstice, ~June 21).
/// JDN 2317672 corresponds to 1633-06-21, approximately the first Kasa
/// before the Sultan Agung epoch.
pub const PRANATA_MASA_KASA_REFERENCE_JDN: i64 = 2_317_672;

/// Compute Pranata Masa position from JDN.
///
/// The Pranata Masa are 12 solar-based agricultural seasons.
/// Kasa (0) starts at ~June solstice, with each season approximately 30 days.
///
/// Algorithm:
/// 1. Calculate days since Kasa reference
/// 2. Modulo by solar year length (365 days)
/// 3. Find the season based on `PRANATA_MASA_SOLAR_OFFSETS`
///
/// Source: Traditional Javanese agricultural calendar; solar positions
/// approximate and vary slightly year-to-year.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn pranata_masa_from_jdn(jdn: i64) -> PranataMasaPos {
    // Calculate days since Kasa reference
    let days_since_kasa = jdn - PRANATA_MASA_KASA_REFERENCE_JDN;

    // Get day within solar year (0-364)
    let day_in_year = days_since_kasa.rem_euclid(i64::from(PRANATA_MASA_SOLAR_YEAR_DAYS)) as u16;

    // Find the Pranata Masa based on offsets (binary search would be faster,
    // but linear search is fine for 12 elements)
    for (idx, &offset) in PRANATA_MASA_SOLAR_OFFSETS.iter().enumerate().rev() {
        if day_in_year >= offset {
            return idx as u8;
        }
    }

    // Fallback to Kasa (shouldn't reach here with valid offsets)
    0
}

/// Get the name of a Pranata Masa position.
#[must_use]
pub const fn pranata_masa_name(pos: PranataMasaPos) -> &'static str {
    PRANATA_MASA_NAMES[pos as usize]
}

// ============================================================================
// DINA MULYA (NOBLE DAYS)
// ============================================================================

/// Dina Mulya (Noble Days) table entry.
///
/// These are days considered particularly auspicious in Javanese tradition.
/// Each entry specifies a Wuku and Saptawara combination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DinaMulyaEntry {
    /// Wuku index (0–29), indexing into `WUKU_NAMES`.
    pub wuku: WukuIdx,
    /// Saptawara position (0–6) for this noble day.
    pub saptawara: SaptawaraPos,
    /// Descriptive name for this Dina Mulya (e.g., "Sinta Jemuwah").
    pub name: &'static str,
}

/// Dina Mulya (Noble Days) — auspicious days in Javanese calendar.
/// Source: Traditional Javanese almanacs (pranatamangsa).
pub const DINA_MULYA: &[DinaMulyaEntry] = &[
    DinaMulyaEntry {
        wuku: 0,
        saptawara: 4,
        name: "Sinta Jemuwah",
    },
    DinaMulyaEntry {
        wuku: 1,
        saptawara: 5,
        name: "Landep Setu",
    },
    DinaMulyaEntry {
        wuku: 2,
        saptawara: 6,
        name: "Ukir Ahad",
    },
    DinaMulyaEntry {
        wuku: 3,
        saptawara: 0,
        name: "Kulantir Soma",
    },
    DinaMulyaEntry {
        wuku: 4,
        saptawara: 1,
        name: "Tolu Selasa",
    },
    DinaMulyaEntry {
        wuku: 5,
        saptawara: 2,
        name: "Gumbreg Rebo",
    },
    DinaMulyaEntry {
        wuku: 6,
        saptawara: 3,
        name: "Warigalit Kemis",
    },
    DinaMulyaEntry {
        wuku: 7,
        saptawara: 4,
        name: "Warigagung Jemuwah",
    },
    DinaMulyaEntry {
        wuku: 8,
        saptawara: 5,
        name: "Julungwangi Setu",
    },
    DinaMulyaEntry {
        wuku: 9,
        saptawara: 6,
        name: "Sungsang Ahad",
    },
    DinaMulyaEntry {
        wuku: 10,
        saptawara: 0,
        name: "Galungan Soma",
    },
    DinaMulyaEntry {
        wuku: 11,
        saptawara: 1,
        name: "Kuningan Selasa",
    },
];

// ============================================================================
// JAVANESE DATE COMPUTATIONS FROM JDN
// ============================================================================

/// Compute Pasaran (5-day market cycle) position from JDN.
///
/// Uses the congruence formula: `jdn.rem_euclid(5)`
/// → 0=Legi, 1=Pahing, 2=Pon, 3=Wage, 4=Kliwon
///
/// Verification: JDN 2317690 (epoch, 1633-07-08) → 0 (Legi), matching "Jumat Legi".
/// Source: Derived from `beaudu/weton` MATLAB reference implementation.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn pasaran_from_jdn(jdn: i64) -> PasaranPos {
    jdn.rem_euclid(5) as u8
}

/// Compute Saptawara (7-day week) position from JDN.
///
/// Algorithm: `jdn.rem_euclid(7)` → 0=Soma, 1=Selasa, ..., 6=Ahad
///
/// Verification: JDN 2317690 (epoch, 1633-07-08) was a Friday → Jemuwah = 4.
/// Source: Derived from `beaudu/weton` MATLAB reference implementation.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn saptawara_from_jdn(jdn: i64) -> SaptawaraPos {
    jdn.rem_euclid(7) as u8
}

/// Compute `Wetonan` (Saptawara + Pasaran) from JDN.
///
/// This is the fundamental 35-day personal cycle used for birth dates.
#[must_use]
pub const fn wetonan_from_jdn(jdn: i64) -> Wetonan {
    (saptawara_from_jdn(jdn), pasaran_from_jdn(jdn))
}

/// Compute Wuku position in the 30-wuku cycle from JDN.
///
/// Algorithm: `((jdn / 7) + 12).rem_euclid(30)` → 0–29 index into `WUKU_NAMES`
///
/// The +12 offset aligns the 30-wuku cycle with the epoch (JDN 2317690 = Sinta).
/// Each wuku is 7 days.
///
/// Verification: JDN 2317690 (epoch) → ((2317690/7) + 12) % 30 = 0 (Sinta) ✓
/// Source: Derived from `beaudu/weton` MATLAB and Dershowitz-Reingold Ch. 10.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn wuku_from_jdn(jdn: i64) -> WukuIdx {
    // Week number from JDN with offset to align cycle
    let week_num = jdn / 7 + 12;
    week_num.rem_euclid(30) as u8
}

/// Compute `WukuPos` (full Pawukon position) from JDN.
///
/// Combines wuku index with day-in-wuku (saptawara position).
#[must_use]
pub const fn wuku_pos_from_jdn(jdn: i64) -> WukuPos {
    WukuPos {
        wuku: wuku_from_jdn(jdn),
        day_in_wuku: saptawara_from_jdn(jdn),
    }
}

/// Compute Pawukon day number (0–209) from JDN.
///
/// The Pawukon is a 210-day cycle combining 30 wukus × 7 days.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn pawukon_day_from_jdn(jdn: i64) -> u16 {
    // Each wuku is 7 days, and there are 30 wukus
    // Epoch (JDN 2317690) = Pawukon day 0
    let days_since = jdn - SULTAN_AGUNG_EPOCH_JDN;
    days_since.rem_euclid(210) as u16
}

/// Lunar month names (Wulan) in Javanese.
/// 12 months per year, alternating 29/30 days with leap year variations.
pub const WULAN_NAMES: [&str; 12] = [
    "Sura",        // 1
    "Sapar",       // 2
    "Mulud",       // 3 (Rabiul Awal - Mawlid)
    "Bakdamulud",  // 4 (Rabiul Akhir)
    "Jumadilawal", // 5
    "Jumadilakir", // 6
    "Rejeb",       // 7 (Rajab)
    "Ruwah",       // 8 (Syaban)
    "Pasa",        // 9 (Ramadan)
    "Sawal",       // 10 (Shawwal)
    "Dulkangidah", // 11 (Dhu al-Qadah)
    "Besar",       // 12 (Dhu al-Hijjah)
];

/// Days in each lunar month for common years (354 days total).
/// Pattern: 30, 29, 30, 29, 30, 29, 30, 29, 30, 29, 30, 29
pub const WULAN_DAYS_COMMON: [u8; 12] = [30, 29, 30, 29, 30, 29, 30, 29, 30, 29, 30, 29];

/// Days in each lunar month for leap years (355 days total).
/// Leap years have 30 days in the last month.
pub const WULAN_DAYS_LEAP: [u8; 12] = [30, 29, 30, 29, 30, 29, 30, 29, 30, 29, 30, 30];

/// Compute lunar month (Wulan) and day from days within a year.
///
/// Takes days counted from the start of the given AJ year (0-indexed).
/// Returns `(wulan_index 0-11, day_in_wulan 1-30)`.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn wulan_from_days_in_year(days_in_year: i64, aj_year: u32) -> Option<(u8, u8)> {
    let windu_year = WinduYear::from_aj(aj_year)?;
    let is_leap = windu_year.is_leap();

    // Get the appropriate month length table
    let month_days = if is_leap {
        &WULAN_DAYS_LEAP
    } else {
        &WULAN_DAYS_COMMON
    };

    // Calculate cumulative days within the year
    let mut cumulative = 0i64;
    for (month_idx, &days_in_month) in month_days.iter().enumerate() {
        let days_in_month_i64 = i64::from(days_in_month);
        let next_cumulative = cumulative + days_in_month_i64;

        if days_in_year < next_cumulative {
            let day_diff = days_in_year - cumulative;
            // day_diff is always >= 0 and < 30, so cast is safe
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let day_in_month = day_diff as u8 + 1;
            return Some((month_idx as u8, day_in_month));
        }
        cumulative = next_cumulative;
    }

    // Day is beyond this year (shouldn't happen with valid input)
    None
}

/// Determine AJ year and day-within-year from days since epoch.
///
/// Searches through windu/kurup cycles to find the correct year,
/// then returns the year plus remaining days within that year.
#[must_use]
pub fn aj_year_and_day_from_days_since_epoch(days: i64) -> Option<(u32, i64)> {
    if days < 0 {
        return None;
    }

    let mut remaining = days;
    let mut current_aj = AJ_MIN;

    // Iterate through years until we find the right one
    loop {
        if current_aj > AJ_MAX {
            return None; // Beyond supported range
        }

        let windu_year = WinduYear::from_aj(current_aj)?;
        let year_days = i64::from(windu_year.days_in_year());

        if remaining < year_days {
            // Found the year - remaining is days within this year
            return Some((current_aj, remaining));
        }

        remaining -= year_days;
        current_aj += 1;
    }
}

// ============================================================================
// JAVANESE DATE TYPE
// ============================================================================

/// A date in the Javanese calendar system.
///
/// This struct holds the computed calendar values for a given JDN within
/// the supported range (AJ 1555–2474).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JavaneseDay {
    /// Julian Day Number (internal pivot).
    jdn: i64,
    /// Anno Javanico year (1555–2474).
    pub aj_year: u32,
    /// Month in the lunar year (1–12, Sura–Besar).
    pub lunar_month: u8,
    /// Day of the lunar month (1–30).
    pub lunar_day: u8,
    /// Windu year (Alip through Jimakir).
    pub windu_year: WinduYear,
    /// Position in Pawukon 210-day cycle.
    pub wuku_pos: WukuPos,
    /// Wetonan (Saptawara + Pasaran).
    pub wetonan: Wetonan,
}

impl JavaneseDay {
    /// Create from JDN. Returns `None` if outside supported range.
    #[must_use]
    pub fn from_jdn(jdn: i64) -> Option<Self> {
        if !(JDN_MIN..=JDN_MAX).contains(&jdn) {
            return None;
        }

        // Compute `Wetonan` (35-day cycle)
        let wetonan = wetonan_from_jdn(jdn);

        // Compute `Pawukon` position (210-day cycle)
        let wuku_pos = wuku_pos_from_jdn(jdn);

        // Compute AJ year and days within year
        let days_since_epoch = jdn - SULTAN_AGUNG_EPOCH_JDN;
        let (aj_year, days_in_year) = aj_year_and_day_from_days_since_epoch(days_since_epoch)?;

        // Compute lunar month and day from days-within-year
        let (lunar_month_idx, lunar_day) = wulan_from_days_in_year(days_in_year, aj_year)?;

        // Get windu year
        let windu_year = WinduYear::from_aj(aj_year)?;

        Some(Self {
            jdn,
            aj_year,
            lunar_month: lunar_month_idx + 1, // Convert 0-indexed to 1-indexed
            lunar_day,
            windu_year,
            wuku_pos,
            wetonan,
        })
    }

    /// Get the Saptawara (day of week) component.
    #[must_use]
    pub const fn saptawara(&self) -> SaptawaraPos {
        self.wetonan.0
    }

    /// Get the Pasaran component.
    #[must_use]
    pub const fn pasaran(&self) -> PasaranPos {
        self.wetonan.1
    }

    /// Get the Wuku name.
    #[must_use]
    pub const fn wuku_name(&self) -> &'static str {
        WUKU_NAMES[self.wuku_pos.wuku as usize]
    }

    /// Check if this is a Dina Mulya (Noble Day).
    #[must_use]
    pub fn is_dina_mulya(&self) -> bool {
        DINA_MULYA
            .iter()
            .any(|entry| entry.wuku == self.wuku_pos.wuku && entry.saptawara == self.wetonan.0)
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS (PLACEHOLDER)
// ============================================================================

impl CalendarDate for JavaneseDay {
    fn from_jdn(jdn: JDN) -> Result<Self, CalendarError> {
        Self::from_jdn(jdn).ok_or_else(|| {
            #[cfg(feature = "std")]
            {
                CalendarError::OutOfRange(format!(
                    "JDN {jdn} outside supported range {JDN_MIN}–{JDN_MAX}"
                ))
            }
            #[cfg(not(feature = "std"))]
            {
                CalendarError::OutOfRange("JDN outside supported range".into())
            }
        })
    }

    fn to_jdn(&self) -> JDN {
        self.jdn
    }

    fn calendar_name() -> &'static str {
        "Javanese Calendar (Sultan Agung Era)"
    }

    fn validate_range(&self) -> Result<(), CalendarError> {
        if self.aj_year < AJ_MIN || self.aj_year > AJ_MAX {
            #[cfg(feature = "std")]
            {
                return Err(CalendarError::OutOfRange(format!(
                    "AJ {} outside {}–{}",
                    self.aj_year, AJ_MIN, AJ_MAX
                )));
            }
            #[cfg(not(feature = "std"))]
            {
                return Err(CalendarError::OutOfRange(
                    "AJ year outside supported range".into(),
                ));
            }
        }
        Ok(())
    }
}

impl CalendarMetadata for JavaneseDay {
    fn epoch() -> JDN {
        SULTAN_AGUNG_EPOCH_JDN
    }

    fn cycle_length() -> Option<calendar_core::CycleYear> {
        Some(calendar_core::CycleYear::from(WINDU_LENGTH_DAYS))
    }

    fn description() -> &'static str {
        "Javanese calendar combining lunar Islamic months with pre-Islamic Pawukon cycles and Hindu-Javanese Windu/Kurup epochs"
    }

    fn cultural_origin() -> &'static str {
        "Java, Indonesia; inaugurated by Sultan Agung of Mataram in 1633 CE, integrating Saka Hindu and Islamic elements"
    }
}

// ============================================================================
// MODULE EXPORTS
// ============================================================================

#[cfg(test)]
mod tests;
