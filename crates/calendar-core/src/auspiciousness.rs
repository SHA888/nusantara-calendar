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
    pub fn description(&self) -> &str {
        match self {
            Activity::Marriage => "Marriage ceremonies and weddings",
            Activity::Building => "Building construction and foundation laying",
            Activity::Travel => "Travel and journeys",
            Activity::Business => "Business ventures and commerce",
            Activity::Agriculture => "Agricultural activities and planting",
            Activity::ReligiousCeremony => "Religious ceremonies and rituals",
            Activity::Naming => "Naming ceremonies for children",
            Activity::MovingHouse => "Moving to a new house or residence",
            Activity::Education => "Starting education or learning",
            Activity::Medical => "Medical treatments and procedures",
            Activity::Custom(desc) => desc,
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
    pub fn description(&self) -> &'static str {
        match self {
            AuspiciousnessLevel::VeryAuspicious => "Very auspicious - extremely favorable",
            AuspiciousnessLevel::Auspicious => "Auspicious - favorable",
            AuspiciousnessLevel::Neutral => "Neutral - neither favorable nor unfavorable",
            AuspiciousnessLevel::Inauspicious => "Inauspicious - unfavorable",
            AuspiciousnessLevel::VeryInauspicious => "Very inauspicious - very unfavorable",
        }
    }
    
    /// Check if the level is auspicious (favorable)
    pub fn is_auspicious(self) -> bool {
        matches!(self, AuspiciousnessLevel::Auspicious | AuspiciousnessLevel::VeryAuspicious)
    }
    
    /// Check if the level is very auspicious
    pub fn is_very_auspicious(self) -> bool {
        matches!(self, AuspiciousnessLevel::VeryAuspicious)
    }
    
    /// Check if the level is inauspicious (unfavorable)
    pub fn is_inauspicious(self) -> bool {
        matches!(self, AuspiciousnessLevel::Inauspicious | AuspiciousnessLevel::VeryInauspicious)
    }
    
    /// Check if the level is very inauspicious
    pub fn is_very_inauspicious(self) -> bool {
        matches!(self, AuspiciousnessLevel::VeryInauspicious)
    }
    
    /// Check if the level is neutral
    pub fn is_neutral(self) -> bool {
        matches!(self, AuspiciousnessLevel::Neutral)
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
