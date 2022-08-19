//! Module defining [CardCode].


/// The internal code of a [Card](super::card::Card).
/// 
/// It is a ASCII string composed of the following segments:
/// - `0..2`: set;
/// - `2..4`: region;
/// - `4..7`: card;
/// - `7..9`: token, never present if the card is [collectible](super::card::Card::collectible).
/// 
/// # Examples
/// 
/// - Evelynn: `06RU025`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CardCode {
    /// The card code as a [`String`].
    pub full: String
}


impl CardCode {
    /// Determines whether the card code is valid or not by checking the following:
    /// 
    /// - that the full [String::is_ascii];
    /// - that it is either 7 or 9 characters long.
    pub fn is_valid(&self) -> bool {
        let is_ascii = self.full.is_ascii();

        let is_long = match self.full.len() {
            7 => true,
            9 => true,
            _ => false,
        };

        is_ascii && is_long
    }

    /// The set segment of the code.
    /// 
    /// In valid codes, it is always 2-ASCII-characters long.
    pub fn set(&self) -> &str {
        &self.full[0..2]
    }

    /// The region segment of the code.
    /// 
    /// In valid codes, it is always 2-ASCII-characters long.
    pub fn region(&self) -> &str {
        &self.full[2..4]
    }

    /// The card segment of the code.
    /// 
    /// In valid codes, it is always 3-ASCII-characters long.
    pub fn card(&self) -> &str {
        &self.full[4..7]
    }

    /// The token segment of the code.
    /// 
    /// In valid codes, it may either be an empty string, or 2-ASCII-characters long.
    pub fn token(&self) -> &str {
        &self.full[7..9]
    }

    /// Create a new card code given the set and region strings and the card number.
    ///
    /// Note: Does not perform any kind of check on the `set` and `region` parameters, and may cause the creation of invalid [`CardCode`]s if misused.
    pub fn from_s_r_c(set: &str, region: &str, card: u32) -> Self {
        CardCode::from(format!("{:02}{}{:03}", &set, &region, &card))
    }
}

impl From<CardCode> for String {
    fn from(cc: CardCode) -> Self {
        cc.full
    }
}

/// Create a new card code given the full card code string.
///
/// Note: Does not perform any kind of check on the given string, and may cause the creation of invalid [`CardCode`]s if misused.
impl From<String> for CardCode {
    fn from(full: String) -> Self {
        CardCode { full }
    }
}
