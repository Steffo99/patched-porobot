//! Module defining [Type].

/// A possible card type.
///
/// Since more types might be added in the future, as it happened with landmarks, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CardType {
    /// A spell.
    Spell,
    /// An unit: either a minion, or a champion.
    /// Champions have their `supertype` set to `Champion`, and their `rarity` set to `Champion` as well.
    Unit,
    /// An ability triggered by an unit.
    Ability,
    /// A landmark.
    Landmark,
    /// A trap or boon.
    Trap,

    /// Unsupported card type.
    #[serde(other)]
    Unsupported,
}