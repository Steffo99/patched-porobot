//! Module defining [CardRegion].

/// A region to which cards can belong to.
///
/// Since more regions might be added in the future, especially Origin ones, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CardRegion {
    /// Noxus.
    Noxus,
    /// Demacia.
    Demacia,
    /// Freljord.
    Freljord,
    /// Shadow Isles.
    ShadowIsles,
    /// Targon.
    Targon,
    /// Ionia.
    Ionia,
    /// Shurima.
    Shurima,
    /// Piltover & Zaun.
    PiltoverZaun,
    /// Bandle City.
    BandleCity,

    /// Runeterra.
    Runeterra,

    /// Origin: The Virtuoso.
    Jhin,
    /// Origin: Agony's Embrace.
    Evelynn,
    /// Origin: The Wandering Caretaker.
    Bard,

    /// Unsupported region.
    #[serde(other)]
    Unsupported,
}
