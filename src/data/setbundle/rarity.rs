//! Module defining [CardRarity].

use crate::data::corebundle::rarity::{LocalizedCardRarity, LocalizedCardRarityIndex};

/// A possible [Card](super::card::Card) rarity.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CardRarity {
    /// The card has no rarity, as it probably is not [collectible](super::card::Card::collectible).
    None,

    /// A common (green triangle) card.
    Common,

    /// A rare (blue square) card.
    Rare,

    /// An epic (purple pentagon) card.
    Epic,

    /// A champion (orange hexagon) card, sometimes referred to as *Legendary*.
    Champion,

    /// Unsupported rarity.
    #[serde(other)]
    Unsupported,
}

impl CardRarity {
    /// Get the [LocalizedCardRarity] associated with this [CardRarity].
    ///
    /// Returns [Option::None] if no matching [LocalizedCardRarity] was found, for example rarities missing from the index.
    ///
    /// Equivalent to calling [LocalizedCardRarityIndex::get].
    pub fn localized<'hm>(
        &self,
        hm: &'hm LocalizedCardRarityIndex,
    ) -> Option<&'hm LocalizedCardRarity> {
        hm.get(self)
    }
}

#[cfg(test)]
mod tests {
    use super::CardRarity;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(
                    serde_json::de::from_str::<'static, CardRarity>($src).unwrap(),
                    $res
                );
            }
        };
    }

    test_deserialization!(deserialize_none, r#""None""#, CardRarity::None);
    test_deserialization!(deserialize_common, r#""Common""#, CardRarity::Common);
    test_deserialization!(deserialize_rare, r#""Rare""#, CardRarity::Rare);
    test_deserialization!(deserialize_epic, r#""Epic""#, CardRarity::Epic);
    test_deserialization!(deserialize_champion, r#""Champion""#, CardRarity::Champion);
    test_deserialization!(deserialize_unsupported, r#""Xyzzy""#, CardRarity::Unsupported);
}
