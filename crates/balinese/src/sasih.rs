//! Sasih (lunar months) implementation for the Balinese calendar

/// The 12 Sasih (lunar months) of the Balinese Saka calendar
///
/// Each Sasih traditionally has 30 days (15 penanggal + 15 panglong).
/// The Sasih system is integrated with the Saka year counting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sasih {
    /// Kasa (1st month, approximately June-July)
    Kasa,
    /// Karo (2nd month, approximately July-August)
    Karo,
    /// Katiga (3rd month, approximately August-September)
    Katiga,
    /// Kapat (4th month, approximately September-October)
    Kapat,
    /// Kalima (5th month, approximately October-November)
    Kalima,
    /// Kanem (6th month, approximately November-December)
    Kanem,
    /// Kapitu (7th month, approximately December-January)
    Kapitu,
    /// Kawolu (8th month, approximately January-February)
    Kawolu,
    /// Kasanga (9th month, approximately February-March)
    Kasanga,
    /// Kadasa (10th month, approximately March-April)
    Kadasa,
    /// Desta (11th month, approximately April-May)
    Desta,
    /// Sada (12th month, approximately May-June)
    Sada,
}

impl Sasih {
    /// Get the traditional Balinese name of this Sasih
    ///
    /// # Returns
    /// The Balinese name as a string slice
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Kasa => "Kasa",
            Self::Karo => "Karo",
            Self::Katiga => "Katiga",
            Self::Kapat => "Kapat",
            Self::Kalima => "Kalima",
            Self::Kanem => "Kanem",
            Self::Kapitu => "Kapitu",
            Self::Kawolu => "Kawolu",
            Self::Kasanga => "Kasanga",
            Self::Kadasa => "Kadasa",
            Self::Desta => "Desta",
            Self::Sada => "Sada",
        }
    }

    /// Get the number of days in this Sasih
    ///
    /// Each Sasih has 30 days: 15 penanggal + 15 panglong per official balinese-calendar crate
    ///
    /// # Returns
    /// Number of days (always 30)
    #[must_use]
    pub const fn days(&self) -> u8 {
        30
    }

    /// Create a Sasih from its index (0-11)
    ///
    /// # Arguments
    /// * `index` - Index in the 12 Sasih cycle
    ///
    /// # Returns
    /// The corresponding Sasih
    ///
    /// # Panics
    /// Panics if index >= 12
    #[must_use]
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Kasa,
            1 => Self::Karo,
            2 => Self::Katiga,
            3 => Self::Kapat,
            4 => Self::Kalima,
            5 => Self::Kanem,
            6 => Self::Kapitu,
            7 => Self::Kawolu,
            8 => Self::Kasanga,
            9 => Self::Kadasa,
            10 => Self::Desta,
            11 => Self::Sada,
            _ => panic!("Sasih index must be 0-11, got {}", index),
        }
    }

    /// Get the index of this Sasih in the 12 Sasih cycle
    ///
    /// # Returns
    /// Index value (0-11)
    #[must_use]
    pub const fn index(&self) -> usize {
        match self {
            Self::Kasa => 0,
            Self::Karo => 1,
            Self::Katiga => 2,
            Self::Kapat => 3,
            Self::Kalima => 4,
            Self::Kanem => 5,
            Self::Kapitu => 6,
            Self::Kawolu => 7,
            Self::Kasanga => 8,
            Self::Kadasa => 9,
            Self::Desta => 10,
            Self::Sada => 11,
        }
    }

    /// Check if this Sasih can have a Nampih (intercalary month)
    ///
    /// Nampih months are added to keep the lunar calendar aligned with solar years
    ///
    /// # Returns
    /// True if this Sasih can have a Nampih month
    pub fn can_be_nampih(&self) -> bool {
        // Traditionally, only certain Sasih can have Nampih months
        matches!(self, Self::Kadasa | Self::Desta)
    }

    /// Get the approximate Gregorian months this Sasih corresponds to
    ///
    /// # Returns
    /// Tuple of (starting_month, ending_month) as Gregorian month numbers
    pub fn approximate_gregorian_months(&self) -> (u8, u8) {
        match self {
            Self::Kasa => (6, 7),     // June-July
            Self::Karo => (7, 8),     // July-August
            Self::Katiga => (8, 9),   // August-September
            Self::Kapat => (9, 10),   // September-October
            Self::Kalima => (10, 11), // October-November
            Self::Kanem => (11, 12),  // November-December
            Self::Kapitu => (12, 1),  // December-January
            Self::Kawolu => (1, 2),   // January-February
            Self::Kasanga => (2, 3),  // February-March
            Self::Kadasa => (3, 4),   // March-April
            Self::Desta => (4, 5),    // April-May
            Self::Sada => (5, 6),     // May-June
        }
    }
}
