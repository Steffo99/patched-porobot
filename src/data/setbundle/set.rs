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

    /// Get the [`CardSet`] from its short code, **assuming it is not an [`CardSet::Event`] card**.
    ///
    /// [`CardSet::Event`] cards have the short code of the set they were released in, so it is impossible to determine if a card belongs to that set from its short code.
    pub fn from_code(value: &str) -> Self {
        match value {
            "01" => Self::Foundations,
            "02" => Self::RisingTides,
            "03" => Self::CallOfTheMountain,
            "04" => Self::EmpiresOfTheAscended,
            "05" => Self::BeyondTheBandlewood,
            "06" => Self::Worldwalker,

            _ => Self::Unsupported,
        }
    }

    /// Get the short code of this [`CardSet`].
    ///
    /// [`CardSet::Event`] cards have the short code of the set they were released in, so this method will return [`Option::None`] for them.
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

            _ => None,
        }
    }

    /// Get the [`CardSet`] from its internal id.
    ///
    /// [`CardSet::Event`] cards have the id of the set they were released in, so it is impossible to determine if a card belongs to that set from its id.
    pub fn from_id(value: u32) -> Self {
        match value {
            1 => Self::Foundations,
            2 => Self::RisingTides,
            3 => Self::CallOfTheMountain,
            4 => Self::EmpiresOfTheAscended,
            5 => Self::BeyondTheBandlewood,
            6 => Self::Worldwalker,
            _ => Self::Unsupported,
        }
    }

    /// Get the internal id of this [`CardSet`].
    ///
    /// If the set has no internal id, it will return [`Option::None`].
    pub fn to_id(&self) -> Option<u32> {
        match self {
            Self::Foundations => Some(1),
            Self::RisingTides => Some(2),
            Self::CallOfTheMountain => Some(3),
            Self::EmpiresOfTheAscended => Some(4),
            Self::BeyondTheBandlewood => Some(5),
            Self::Worldwalker => Some(6),
            _ => None,
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
    test_deserialization!(deserialize_setevent, r#""SetEvent""#, CardSet::Events);
    test_deserialization!(deserialize_fallback, r#""Xyzzy""#, CardSet::Unsupported);
}
