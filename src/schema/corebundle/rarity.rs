//! This module defines [CoreRarity].

use crate::schema::setbundle::CardRarity;

/// A Legends of Runeterra [CardRarity], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CoreRarity {
    /// The [CardRarity] these strings refer to.
    #[serde(rename = "nameRef")]
    pub rarity: CardRarity,

    /// The localized name of the rarity.
    pub name: String,
}
