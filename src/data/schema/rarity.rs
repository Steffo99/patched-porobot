//! Module defining [CardRarity].

/// A possible card rarity.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CardRarity {
    /// The card has no rarity, as it probably is not collectible.
    #[serde(alias = "NONE")]
    None,
    /// A common card.
    #[serde(alias = "COMMON")]
    Common,
    /// A rare card.
    #[serde(alias = "RARE")]
    Rare,
    /// An epic card.
    #[serde(alias = "EPIC")]
    Epic,
    /// A champion.
    #[serde(alias = "CHAMPION")]
    Champion,
}
