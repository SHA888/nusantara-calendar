//! Wewaran (week cycles) implementation for the Balinese calendar

/// Saptawara (7-day week) - corresponds to the traditional week
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Saptawara {
    /// Redite (Sunday)
    Redite,
    /// Soma (Monday)
    Soma,
    /// Anggara (Tuesday)
    Anggara,
    /// Buda (Wednesday)
    Buda,
    /// Wraspati/Respati (Thursday)
    Wraspati,
    /// Sukra (Friday)
    Sukra,
    /// Saniscara (Saturday)
    Saniscara,
}

impl Saptawara {
    /// Get the traditional Balinese name of this day
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Redite => "Redite",
            Self::Soma => "Soma",
            Self::Anggara => "Anggara",
            Self::Buda => "Buda",
            Self::Wraspati => "Wraspati",
            Self::Sukra => "Sukra",
            Self::Saniscara => "Saniscara",
        }
    }
    
    /// Get the urip (numerical value) of this day
    #[must_use]
    pub const fn urip(&self) -> u8 {
        match self {
            Self::Redite => 5,
            Self::Soma => 4,
            Self::Anggara => 3,
            Self::Buda => 7,
            Self::Wraspati => 8,
            Self::Sukra => 6,
            Self::Saniscara => 9,
        }
    }
    
    /// Create from index (0-6, where 0 = Redite/Sunday)
    #[must_use]
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Redite,
            1 => Self::Soma,
            2 => Self::Anggara,
            3 => Self::Buda,
            4 => Self::Wraspati,
            5 => Self::Sukra,
            6 => Self::Saniscara,
            _ => panic!("Saptawara index must be 0-6, got {}", index),
        }
    }
    
    /// Get the index of this day
    #[must_use]
    pub const fn index(&self) -> usize {
        match self {
            Self::Redite => 0,
            Self::Soma => 1,
            Self::Anggara => 2,
            Self::Buda => 3,
            Self::Wraspati => 4,
            Self::Sukra => 5,
            Self::Saniscara => 6,
        }
    }
}

/// Pancawara (5-day market week) - traditional market cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pancawara {
    /// Paing
    Paing,
    /// Pon
    Pon,
    /// Wage
    Wage,
    /// Kliwon
    Kliwon,
    /// Umanis
    Umanis,
}

impl Pancawara {
    /// Get the traditional Balinese name of this day
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Paing => "Paing",
            Self::Pon => "Pon",
            Self::Wage => "Wage",
            Self::Kliwon => "Kliwon",
            Self::Umanis => "Umanis",
        }
    }
    
    /// Get the urip (numerical value) of this day
    #[must_use]
    pub const fn urip(&self) -> u8 {
        match self {
            Self::Paing => 9,
            Self::Pon => 7,
            Self::Wage => 4,
            Self::Kliwon => 8,
            Self::Umanis => 5,
        }
    }
    
    /// Create from index (0-4, where 0 = Paing)
    #[must_use]
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Paing,
            1 => Self::Pon,
            2 => Self::Wage,
            3 => Self::Kliwon,
            4 => Self::Umanis,
            _ => panic!("Pancawara index must be 0-4, got {}", index),
        }
    }
    
    /// Get the index of this day
    #[must_use]
    pub const fn index(&self) -> usize {
        match self {
            Self::Paing => 0,
            Self::Pon => 1,
            Self::Wage => 2,
            Self::Kliwon => 3,
            Self::Umanis => 4,
        }
    }
}
