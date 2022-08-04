//! Module defining [CardRarity].


use std::collections::HashMap;

use crate::schema::corebundle::CoreRarity;

/// A possible [super::Card] rarity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CardRarity {
    /// The card has no rarity, as it probably is not collectible.
    None,

    /// A common card.
    Common,

    /// A rare card.
    Rare,

    /// An epic card.
    Epic,

    /// A champion.
    Champion,
}


impl CardRarity {
    /// Get localized text about the rarity from [crate::schema::corebundle] data.
    ///
    /// Returns `None` if no matching [CoreRarity] was found, for example if the hashmap is incomplete.
    pub fn localized<'hm>(&self, hm: &'hm HashMap<CardRarity, CoreRarity>) -> Option<&'hm CoreRarity> {
        hm.get(&self)
    }
}


#[cfg(test)]
mod tests {
    use super::CardRarity;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(serde_json::de::from_str::<'static, CardRarity>($src).unwrap(), $res);
            }
        }
    }

    test_deserialization!(deserialize_none, r#""None""#, CardRarity::None);
    test_deserialization!(deserialize_common, r#""Common""#, CardRarity::Common);
    test_deserialization!(deserialize_rare, r#""Rare""#, CardRarity::Rare);
    test_deserialization!(deserialize_epic, r#""Epic""#, CardRarity::Epic);
    test_deserialization!(deserialize_champion, r#""Champion""#, CardRarity::Champion);

    #[test]
    fn deserialize_fallback() {
        assert!(serde_json::de::from_str::<'static, CardRarity>("Xyzzy").is_err());
    }
}
