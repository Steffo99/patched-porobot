//! Module defining structs representing localized card rarities.

use crate::data::setbundle::rarity::CardRarity;
use std::collections::HashMap;

/// A Legends of Runeterra [CardRarity], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LocalizedCardRarity {
    /// The [CardRarity] these strings refer to.
    #[serde(rename = "nameRef")]
    pub rarity: CardRarity,

    /// The localized name of the rarity.
    pub name: String,
}

/// How [LocalizedCardRarity]s appear in `global.json` files.
pub type LocalizedCardRarityVec = Vec<LocalizedCardRarity>;
/// An index of [LocalizedCardRarity]s, with [LocalizedCardRarity::rarity]s as keys.
pub type LocalizedCardRarityIndex = HashMap<CardRarity, LocalizedCardRarity>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, LocalizedCardRarity>(r#"
                {
                    "name": "COMMON",
                    "nameRef": "Common"
                }
            "#).unwrap(),
            LocalizedCardRarity {
                rarity: CardRarity::Common,
                name: "COMMON".to_string(),
            }
        );
    }
}
