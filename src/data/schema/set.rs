//! Module defining [CardSet].

/// The release set a [Card] may belong to.
///
/// Since more sets will definitely be added in the future, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
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
