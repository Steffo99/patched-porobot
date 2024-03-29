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

    /// Get the Discord emoji code associated with this [`CardRarity`].
    pub fn discord_emoji(&self) -> &'static str {
        match self {
            CardRarity::None => "",
            CardRarity::Common => "<:common:1056024315046412358>",
            CardRarity::Rare => "<:rare:1056022907433799690>",
            CardRarity::Epic => "<:epic:1056023004028608622>",
            CardRarity::Champion => "<:champion:1056024303856001034>",
            CardRarity::Unsupported => "",
        }
    }

    /// Get the color associated with this [`CardRarity`].
    ///
    /// Used for example to determine the color of the Discord embed.
    pub fn color(&self) -> u32 {
        match self {
            CardRarity::None => 0x202225,
            CardRarity::Common => 0x1e6a49,
            CardRarity::Rare => 0x244778,
            CardRarity::Epic => 0x502970,
            CardRarity::Champion => 0x81541f,
            CardRarity::Unsupported => 0xff0000,
        }
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
