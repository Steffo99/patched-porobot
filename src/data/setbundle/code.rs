//! Module defining [CardCode].

use crate::data::setbundle::card::{Card, CardIndex};

/// The internal code of a [`Card`].
///
/// It is a ASCII string composed of the following segments:
/// - `0..2`: set;
/// - `2..4`: region;
/// - `4..7`: card;
/// - `7..9`: token, never present if the card is [collectible](super::card::Card::collectible).
///
/// # Example
///
/// ```rust
/// use patched_porobot::data::setbundle::code::CardCode;
///
/// CardCode { full: "06RU025".to_string() };
/// ```
///
/// # Warning
///
/// The way this is built is pretty... unsafe, so beware to not construct this with invalid codes.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct CardCode {
    /// The card code as a [`String`].
    pub full: String,
}

impl CardCode {
    /// Determines whether the card code is valid or not by checking the following:
    ///
    /// - that the full string [is ascii](str::is_ascii);
    /// - that it is either 7 or 9 characters long.
    pub fn is_valid(&self) -> bool {
        let is_ascii = self.full.is_ascii();

        let is_long = matches!(self.full.len(), 7 | 9);

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
    pub fn token(&self) -> Option<&str> {
        if self.full.len() >= 9 {
            Some(&self.full[7..9])
        }
        else {
            None
        }
    }

    /// Create a new card code given the set and region strings and the card number.
    ///
    /// Note: Does not perform any kind of check on the `set` and `region` parameters, and may cause the creation of invalid [`CardCode`]s if misused.
    pub fn from_s_r_c(set: &str, region: &str, card: u32) -> Self {
        CardCode::from(format!("{:02}{}{:03}", &set, &region, &card))
    }

    /// Find, in a [`CardIndex`], the [`Card`] this code belongs to.
    pub fn to_card<'c>(&self, cards: &'c CardIndex) -> Option<&'c Card> {
        cards.get(self)
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

/// Extract the card code from a [`Card`].
impl From<Card> for CardCode {
    fn from(c: Card) -> Self {
        c.code
    }
}