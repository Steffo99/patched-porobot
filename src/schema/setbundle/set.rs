//! Module defining [CardSet].


use std::collections::HashMap;

use crate::schema::corebundle::CoreSet;

/// The release set a [super::Card] may belong to.
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

    /// Events, with cards released "outside" a set.
    #[serde(rename = "SetEvent")]
    Events,

    /// Unsupported card set.
    #[serde(other)]
    Unsupported,
}


impl CardSet {
    /// Get localized text about the set from [crate::schema::corebundle] data.
    ///
    /// Returns `None` if no matching [CoreSet] was found, for example for [CardSet::Unsupported] sets.
    pub fn localized<'hm>(&self, hm: &'hm HashMap<CardSet, CoreSet>) -> Option<&'hm CoreSet> {
        hm.get(&self)
    }
}


#[cfg(test)]
mod tests {
    use super::CardSet;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(serde_json::de::from_str::<'static, CardSet>($src).unwrap(), $res);
            }
        }
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
