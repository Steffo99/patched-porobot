//! Module defining [CardSet].

use crate::data::corebundle::set::{LocalizedCardSet, LocalizedCardSetIndex};

/// The release set a [Card](super::card::Card) may belong to.
///
/// Since more sets will definitely be added in the future, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CardSet {
    /// Foundations, or "base".
    #[serde(rename = "Set1")]
    Foundations,

    /// Rising Tides.
    #[serde(rename = "Set2")]
    RisingTides,

    /// Call of the Mountain.
    #[serde(rename = "Set3")]
    CallOfTheMountain,

    /// Empires of the Ascended.
    #[serde(rename = "Set4")]
    EmpiresOfTheAscended,

    /// Beyond the Bandlewood.
    #[serde(rename = "Set5")]
    BeyondTheBandlewood,

    /// Worldwalker.
    #[serde(rename = "Set6")]
    Worldwalker,

    /// The Darkin Saga / Awakening.
    #[serde(rename = "Set6cde")]
    TheDarkinSaga,

    /// Glory in Navori.
    #[serde(rename = "Set7")]
    GloryInNavori,

    /// Events, cards released "outside" a set.
    #[serde(rename = "SetEvent")]
    Events,

    /// Unsupported card set.
    #[serde(other)]
    Unsupported,
}

impl CardSet {
    /// Get the [LocalizedCardSet] associated with this [CardSet].
    ///
    /// Returns [Option::None] if no matching [LocalizedCardSet] was found, for example for [CardSet::Unsupported] sets.
    ///
    /// Equivalent to calling [LocalizedCardSetIndex::get].
    pub fn localized<'hm>(&self, hm: &'hm LocalizedCardSetIndex) -> Option<&'hm LocalizedCardSet> {
        hm.get(self)
    }

    /// Get the [`CardSet`] from its short code.
    ///
    /// [`CardSet::Worldwalker`] and [`CardSet::TheDarkinSaga`] share the same code `06`, so a variant cannot be determined.
    /// 
    /// [`CardSet::Events`] cards have the short code of the set they were released in, so it is impossible to determine if a card belongs to that set from its short code.
    pub fn from_code(value: &str) -> Self {
        match value {
            "01" => Self::Foundations,
            "02" => Self::RisingTides,
            "03" => Self::CallOfTheMountain,
            "04" => Self::EmpiresOfTheAscended,
            "05" => Self::BeyondTheBandlewood,

            _ => Self::Unsupported,
        }
    }

    /// Get the short code of this [`CardSet`].
    ///
    /// [`CardSet::Events`] cards have the short code of the set they were released in, so this method will return [`Option::None`] for them.
    ///
    /// If the set has no short code, it will also return [`Option::None`].
    pub fn to_code(&self) -> Option<String> {
        match self {
            Self::Foundations => Some("01".to_string()),
            Self::RisingTides => Some("02".to_string()),
            Self::CallOfTheMountain => Some("03".to_string()),
            Self::EmpiresOfTheAscended => Some("04".to_string()),
            Self::BeyondTheBandlewood => Some("05".to_string()),
            Self::Worldwalker => Some("06".to_string()),
            Self::TheDarkinSaga => Some("06".to_string()),

            _ => None,
        }
    }
}

/// Get the [`CardSet`] from its internal id.
///
/// [`CardSet::Worldwalker`] and [`CardSet::TheDarkinSaga`] share the same id, so a variant cannot be determined.
/// 
/// [`CardSet::Events`] cards have the id of the set they were released in, so it is impossible to determine if a card belongs to that set from its id.
impl From<u32> for CardSet {
    fn from(value: u32) -> Self {
        match value {
            1 => CardSet::Foundations,
            2 => CardSet::RisingTides,
            3 => CardSet::CallOfTheMountain,
            4 => CardSet::EmpiresOfTheAscended,
            5 => CardSet::BeyondTheBandlewood,
            _ => CardSet::Unsupported,
        }
    }
}

/// Get the internal id of this [`CardSet`].
///
/// If the set has no associated internal id, it will return [`Result::Err`].
impl TryFrom<CardSet> for u32 {
    type Error = ();

    fn try_from(value: CardSet) -> Result<Self, Self::Error> {
        match value {
            CardSet::Foundations => Ok(1),
            CardSet::RisingTides => Ok(2),
            CardSet::CallOfTheMountain => Ok(3),
            CardSet::EmpiresOfTheAscended => Ok(4),
            CardSet::BeyondTheBandlewood => Ok(5),
            CardSet::Worldwalker => Ok(6),
            CardSet::TheDarkinSaga => Ok(6),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CardSet;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(
                    serde_json::de::from_str::<'static, CardSet>($src).unwrap(),
                    $res
                );
            }
        };
    }

    test_deserialization!(deserialize_set1, r#""Set1""#, CardSet::Foundations);
    test_deserialization!(deserialize_set2, r#""Set2""#, CardSet::RisingTides);
    test_deserialization!(deserialize_set3, r#""Set3""#, CardSet::CallOfTheMountain);
    test_deserialization!(deserialize_set4, r#""Set4""#, CardSet::EmpiresOfTheAscended);
    test_deserialization!(deserialize_set5, r#""Set5""#, CardSet::BeyondTheBandlewood);
    test_deserialization!(deserialize_set6, r#""Set6""#, CardSet::Worldwalker);
    test_deserialization!(deserialize_set6cde, r#""Set6cde""#, CardSet::TheDarkinSaga);
    test_deserialization!(deserialize_setevent, r#""SetEvent""#, CardSet::Events);
    test_deserialization!(deserialize_fallback, r#""Xyzzy""#, CardSet::Unsupported);
}
