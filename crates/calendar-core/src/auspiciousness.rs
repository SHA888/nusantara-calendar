//! Auspiciousness system for Indonesian calendars
//!
//! This module defines the activity types and auspiciousness levels
//! used across various Indonesian calendar systems.

use alloc::string::String;

/// Activities that can be evaluated for auspiciousness
///
/// These represent common activities in Indonesian culture that
/// may be scheduled based on calendar auspiciousness.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Activity {
    /// Marriage ceremonies
    Marriage,
    /// Building construction
    Building,
    /// Travel or journeys
    Travel,
    /// Business ventures
    Business,
    /// Agricultural activities
    Agriculture,
    /// Religious ceremonies
    ReligiousCeremony,
    /// Naming ceremonies
    Naming,
    /// Moving to new house
    MovingHouse,
    /// Starting education
    Education,
    /// Medical treatments
    Medical,
    /// Custom activity with description
    Custom(String),
}

impl Activity {
    /// Get a human-readable description of the activity
    #[must_use]
    pub fn description(&self) -> &str {
        match self {
            Self::Marriage => "Marriage ceremonies and weddings",
            Self::Building => "Building construction and foundation laying",
            Self::Travel => "Travel and journeys",
            Self::Business => "Business ventures and commerce",
            Self::Agriculture => "Agricultural activities and planting",
            Self::ReligiousCeremony => "Religious ceremonies and rituals",
            Self::Naming => "Naming ceremonies for children",
            Self::MovingHouse => "Moving to a new house or residence",
            Self::Education => "Starting education or learning",
            Self::Medical => "Medical treatments and procedures",
            Self::Custom(desc) => desc,
        }
    }
}

/// Auspiciousness levels for calendar days
///
/// These levels indicate how favorable a day is for various activities
/// in Indonesian cultural traditions.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AuspiciousnessLevel {
    /// Extremely auspicious - very favorable
    VeryAuspicious,
    /// Auspicious - favorable
    Auspicious,
    /// Neutral - neither favorable nor unfavorable
    Neutral,
    /// Inauspicious - unfavorable
    Inauspicious,
    /// Very inauspicious - very unfavorable
    VeryInauspicious,
}

impl AuspiciousnessLevel {
    /// Get a human-readable description of the level
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::VeryAuspicious => "Very auspicious - extremely favorable",
            Self::Auspicious => "Auspicious - favorable",
            Self::Neutral => "Neutral - neither favorable nor unfavorable",
            Self::Inauspicious => "Inauspicious - unfavorable",
            Self::VeryInauspicious => "Very inauspicious - very unfavorable",
        }
    }

    /// Check if the level is auspicious (favorable)
    #[must_use]
    pub const fn is_auspicious(self) -> bool {
        matches!(self, Self::Auspicious | Self::VeryAuspicious)
    }

    /// Check if the level is very auspicious
    #[must_use]
    pub const fn is_very_auspicious(self) -> bool {
        matches!(self, Self::VeryAuspicious)
    }

    /// Check if the level is inauspicious (unfavorable)
    #[must_use]
    pub const fn is_inauspicious(self) -> bool {
        matches!(self, Self::Inauspicious | Self::VeryInauspicious)
    }

    /// Check if the level is very inauspicious
    #[must_use]
    pub const fn is_very_inauspicious(self) -> bool {
        matches!(self, Self::VeryInauspicious)
    }

    /// Check if the level is neutral
    #[must_use]
    pub const fn is_neutral(self) -> bool {
        matches!(self, Self::Neutral)
    }
}

impl core::fmt::Display for Activity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl core::fmt::Display for AuspiciousnessLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}
