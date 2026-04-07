//! Wariga (auspiciousness calculations) implementation for the Balinese calendar

use alloc::{string::String, vec, vec::Vec};

/// Balinese auspiciousness levels
///
/// These levels correspond to the traditional Balinese concepts of
/// auspicious and inauspicious days for various activities.
pub type AuspiciousnessLevel = calendar_core::AuspiciousnessLevel;

/// Activities that can be evaluated for auspiciousness in Balinese culture
///
/// These represent the most common activities for which Balinese people
/// consult the calendar to determine auspicious timing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Activity {
    /// Marriage ceremonies (Pawiwahan)
    Marriage,
    /// Building construction (Gedong)
    Building,
    /// Religious ceremonies (Yadnya)
    Ceremony,
    /// Travel/Journey (Dolan)
    Travel,
    /// Business ventures (Usaha)
    Business,
    /// Agricultural activities (Tani)
    Agriculture,
    /// Education/Learning (Sekolah)
    Education,
    /// Healing/Medical treatment (Usada)
    Healing,
    /// Tooth filing ceremonies (Mapandes)
    ToothFiling,
    /// Cremation ceremonies (Pitra Yadnya)
    Cremation,
    /// Custom activity with description
    Custom(String),
}

impl Activity {
    /// Get a description of this activity
    #[must_use]
    pub fn description(&self) -> &str {
        match self {
            Self::Marriage => "Marriage ceremony and wedding",
            Self::Building => "Building construction and foundation laying",
            Self::Ceremony => "Religious ceremonies and rituals",
            Self::Travel => "Travel and journeys",
            Self::Business => "Business ventures and commerce",
            Self::Agriculture => "Agricultural activities and planting",
            Self::Education => "Education, learning, and initiation",
            Self::Healing => "Healing and medical treatment",
            Self::ToothFiling => "Tooth filing ceremony (Mapandes)",
            Self::Cremation => "Cremation ceremony (Pitra Yadnya)",
            Self::Custom(desc) => desc,
        }
    }
}

/// Wariga calculation results for auspiciousness evaluation
///
/// This represents the traditional Balinese system of determining
/// the quality of a day for specific activities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarigaResult {
    /// Overall auspiciousness level
    pub auspiciousness: AuspiciousnessLevel,
    /// Specific factors that influenced the calculation
    pub factors: Vec<WarigaFactor>,
    /// Recommended timing within the day
    pub timing: Option<DayTiming>,
}

/// Factors that influence Wariga calculations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WarigaFactor {
    /// Saptawara influence (7-day week)
    Saptawara,
    /// Pancawara influence (5-day week)
    Pancawara,
    /// Wuku influence (30-week cycle)
    Wuku,
    /// Lunar phase (Purnama/Tilem)
    LunarPhase,
    /// Nampih month influence
    Nampih,
    /// Planetary position
    Planetary,
    /// Traditional omen or sign
    Omen,
}

/// Recommended timing within a day
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DayTiming {
    /// Early morning (before sunrise)
    EarlyMorning,
    /// Morning (sunrise to noon)
    Morning,
    /// Afternoon (noon to sunset)
    Afternoon,
    /// Evening (sunset to midnight)
    Evening,
    /// Night (midnight to sunrise)
    Night,
    /// Any time is suitable
    AnyTime,
}

/// Calculate auspiciousness for a specific activity on a given date
///
/// This is a simplified implementation. In practice, Wariga calculations
/// involve complex traditional algorithms and expert knowledge.
///
/// # Arguments
/// * `jdn` - Julian Day Number for the date
/// * `activity` - The activity to evaluate
///
/// # Returns
/// Wariga calculation result with auspiciousness level and factors
pub fn calculate_auspiciousness(jdn: i64, activity: &Activity) -> WarigaResult {
    use crate::balinese_date::BalineseDate;
    use calendar_core::CalendarDate;

    // Create a BalineseDate to get the necessary components
    let Ok(date) = BalineseDate::from_jdn(jdn) else {
        return WarigaResult {
            auspiciousness: AuspiciousnessLevel::Neutral,
            factors: vec![WarigaFactor::Omen],
            timing: None,
        };
    };

    let mut factors = Vec::new();
    let mut score = 0; // Higher is more auspicious

    // Saptawara influence
    match activity {
        Activity::Marriage => match date.saptawara {
            crate::wewaran::Saptawara::Buda | crate::wewaran::Saptawara::Sukra => {
                score += 3;
                factors.push(WarigaFactor::Saptawara);
            }
            crate::wewaran::Saptawara::Redite | crate::wewaran::Saptawara::Saniscara => {
                score -= 2;
                factors.push(WarigaFactor::Saptawara);
            }
            _ => {}
        },
        Activity::Ceremony => {
            if date.is_purnama {
                score += 4;
                factors.push(WarigaFactor::LunarPhase);
            }
        }
        Activity::Building => {
            if date.is_tilem {
                score -= 3;
                factors.push(WarigaFactor::LunarPhase);
            }
        }
        _ => {}
    }

    // Pancawara influence
    match (activity, date.pancawara) {
        (Activity::Business, crate::wewaran::Pancawara::Kliwon)
        | (Activity::Agriculture, crate::wewaran::Pancawara::Umanis) => {
            score += 2;
            factors.push(WarigaFactor::Pancawara);
        }
        _ => {}
    }

    // Wuku influence
    match date.wuku {
        crate::pawukon::Wuku::Sinta | crate::pawukon::Wuku::Landep => {
            score += 1;
            factors.push(WarigaFactor::Wuku);
        }
        crate::pawukon::Wuku::Bala | crate::pawukon::Wuku::Parangbakal => {
            score -= 1;
            factors.push(WarigaFactor::Wuku);
        }
        _ => {}
    }

    // Nampih month influence
    if date.is_nampih {
        score -= 1;
        factors.push(WarigaFactor::Nampih);
    }

    // Convert score to auspiciousness level
    let auspiciousness = match score {
        score if score >= 4 => AuspiciousnessLevel::VeryAuspicious,
        score if score >= 2 => AuspiciousnessLevel::Auspicious,
        score if score >= 0 => AuspiciousnessLevel::Neutral,
        score if score >= -2 => AuspiciousnessLevel::Inauspicious,
        _ => AuspiciousnessLevel::VeryInauspicious,
    };

    // Determine optimal timing
    let timing = match activity {
        Activity::Ceremony => Some(DayTiming::Morning),
        Activity::Marriage => Some(DayTiming::Afternoon),
        Activity::Building => Some(DayTiming::EarlyMorning),
        _ => Some(DayTiming::AnyTime),
    };

    WarigaResult {
        auspiciousness,
        factors,
        timing,
    }
}
