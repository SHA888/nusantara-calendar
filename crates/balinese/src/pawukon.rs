//! Pawukon cycle implementation (30 Wuku × 7 days = 210-day cycle)

/// The 30 Wuku of the Balinese Pawukon cycle
///
/// Each Wuku lasts 7 days, and the complete cycle repeats every 210 days.
/// The Wuku are the foundation of the Balinese calendar system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Wuku {
    /// Sinta (1st Wuku)
    Sinta,
    /// Landep (2nd Wuku)
    Landep,
    /// Ukir (3rd Wuku)
    Ukir,
    /// Kulantir (4th Wuku)
    Kulantir,
    /// Tolu (5th Wuku)
    Tolu,
    /// Gumbreg (6th Wuku)
    Gumbreg,
    /// Wariga (7th Wuku)
    Wariga,
    /// Warigadian (8th Wuku)
    Warigadian,
    /// Julungwangi (9th Wuku)
    Julungwangi,
    /// Sungsang (10th Wuku)
    Sungsang,
    /// Dungulan (11th Wuku)
    Dungulan,
    /// Kuningan (12th Wuku)
    Kuningan,
    /// Langkir (13th Wuku)
    Langkir,
    /// Medangsia (14th Wuku)
    Medangsia,
    /// Pujut (15th Wuku)
    Pujut,
    /// Pahang (16th Wuku)
    Pahang,
    /// Krulut (17th Wuku)
    Krulut,
    /// Merakih (18th Wuku)
    Merakih,
    /// Tambir (19th Wuku)
    Tambir,
    /// Medangkungan (20th Wuku)
    Medangkungan,
    /// Matal (21st Wuku)
    Matal,
    /// Uye (22nd Wuku)
    Uye,
    /// Menail (23rd Wuku)
    Menail,
    /// Parangbakal (24th Wuku)
    Parangbakal,
    /// Bala (25th Wuku)
    Bala,
    /// Ugu (26th Wuku)
    Ugu,
    /// Wayang (27th Wuku)
    Wayang,
    /// Kelawu (28th Wuku)
    Kelawu,
    /// Dukut (29th Wuku)
    Dukut,
    /// Watugunung (30th Wuku)
    Watugunung,
}

impl Wuku {
    /// Get the traditional Balinese name of this Wuku
    ///
    /// # Returns
    /// The Balinese name as a string slice
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Sinta => "Sinta",
            Self::Landep => "Landep",
            Self::Ukir => "Ukir",
            Self::Kulantir => "Kulantir",
            Self::Tolu => "Tolu",
            Self::Gumbreg => "Gumbreg",
            Self::Wariga => "Wariga",
            Self::Warigadian => "Warigadian",
            Self::Julungwangi => "Julungwangi",
            Self::Sungsang => "Sungsang",
            Self::Dungulan => "Dungulan",
            Self::Kuningan => "Kuningan",
            Self::Langkir => "Langkir",
            Self::Medangsia => "Medangsia",
            Self::Pujut => "Pujut",
            Self::Pahang => "Pahang",
            Self::Krulut => "Krulut",
            Self::Merakih => "Merakih",
            Self::Tambir => "Tambir",
            Self::Medangkungan => "Medangkungan",
            Self::Matal => "Matal",
            Self::Uye => "Uye",
            Self::Menail => "Menail",
            Self::Parangbakal => "Parangbakal",
            Self::Bala => "Bala",
            Self::Ugu => "Ugu",
            Self::Wayang => "Wayang",
            Self::Kelawu => "Kelawu",
            Self::Dukut => "Dukut",
            Self::Watugunung => "Watugunung",
        }
    }
    
    /// Get the urip (numerical value) of this Wuku
    ///
    /// Each Wuku has a traditional urip value used in calculations
    ///
    /// # Returns
    /// The urip value (0-9)
    #[must_use]
    pub const fn urip(&self) -> u8 {
        match self {
            Self::Sinta => 5,
            Self::Landep => 3,
            Self::Ukir => 7,
            Self::Kulantir => 6,
            Self::Tolu => 2,
            Self::Gumbreg => 4,
            Self::Wariga => 8,
            Self::Warigadian => 1,
            Self::Julungwangi => 5,
            Self::Sungsang => 3,
            Self::Dungulan => 7,
            Self::Kuningan => 6,
            Self::Langkir => 2,
            Self::Medangsia => 4,
            Self::Pujut => 8,
            Self::Pahang => 1,
            Self::Krulut => 5,
            Self::Merakih => 3,
            Self::Tambir => 7,
            Self::Medangkungan => 6,
            Self::Matal => 2,
            Self::Uye => 4,
            Self::Menail => 8,
            Self::Parangbakal => 1,
            Self::Bala => 5,
            Self::Ugu => 3,
            Self::Wayang => 7,
            Self::Kelawu => 6,
            Self::Dukut => 2,
            Self::Watugunung => 4,
        }
    }
    
    /// Create a Wuku from its index (0-29)
    ///
    /// # Arguments
    /// * `index` - Index in the 30 Wuku cycle
    ///
    /// # Returns
    /// The corresponding Wuku
    ///
    /// # Panics
    /// Panics if index >= 30
    #[must_use]
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Sinta,
            1 => Self::Landep,
            2 => Self::Ukir,
            3 => Self::Kulantir,
            4 => Self::Tolu,
            5 => Self::Gumbreg,
            6 => Self::Wariga,
            7 => Self::Warigadian,
            8 => Self::Julungwangi,
            9 => Self::Sungsang,
            10 => Self::Dungulan,
            11 => Self::Kuningan,
            12 => Self::Langkir,
            13 => Self::Medangsia,
            14 => Self::Pujut,
            15 => Self::Pahang,
            16 => Self::Krulut,
            17 => Self::Merakih,
            18 => Self::Tambir,
            19 => Self::Medangkungan,
            20 => Self::Matal,
            21 => Self::Uye,
            22 => Self::Menail,
            23 => Self::Parangbakal,
            24 => Self::Bala,
            25 => Self::Ugu,
            26 => Self::Wayang,
            27 => Self::Kelawu,
            28 => Self::Dukut,
            29 => Self::Watugunung,
            _ => panic!("Wuku index must be 0-29, got {}", index),
        }
    }
    
    /// Get the index of this Wuku in the 30 Wuku cycle
    ///
    /// # Returns
    /// Index value (0-29)
    #[must_use]
    pub const fn index(&self) -> usize {
        match self {
            Self::Sinta => 0,
            Self::Landep => 1,
            Self::Ukir => 2,
            Self::Kulantir => 3,
            Self::Tolu => 4,
            Self::Gumbreg => 5,
            Self::Wariga => 6,
            Self::Warigadian => 7,
            Self::Julungwangi => 8,
            Self::Sungsang => 9,
            Self::Dungulan => 10,
            Self::Kuningan => 11,
            Self::Langkir => 12,
            Self::Medangsia => 13,
            Self::Pujut => 14,
            Self::Pahang => 15,
            Self::Krulut => 16,
            Self::Merakih => 17,
            Self::Tambir => 18,
            Self::Medangkungan => 19,
            Self::Matal => 20,
            Self::Uye => 21,
            Self::Menail => 22,
            Self::Parangbakal => 23,
            Self::Bala => 24,
            Self::Ugu => 25,
            Self::Wayang => 26,
            Self::Kelawu => 27,
            Self::Dukut => 28,
            Self::Watugunung => 29,
        }
    }
}
