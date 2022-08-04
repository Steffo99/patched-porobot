//! This module defines [CoreRegion].

use crate::schema::setbundle::CardRegion;

/// A Legends of Runeterra [CardRegion], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CoreRegion {
    /// The [CardRegion] these strings refer to.
    #[serde(rename = "nameRef")]
    pub region: CardRegion,

    /// The localized name of the region.
    pub name: String,

    /// The abbreviation for the region.
    ///
    /// Usually two letters long, but may be longer for "Origin" regions.
    pub abbreviation: String,

    /// URL to the icon of the region in `.png` format.
    #[serde(rename = "iconAbsolutePath")]
    pub icon_png: String,
}
