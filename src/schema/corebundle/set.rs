//! This module defines [CoreSet].

use crate::schema::setbundle::CardSet;

/// A Legends of Runeterra [CardSet], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CoreSet {
    /// The [CardSet] these strings refer to.
    #[serde(rename = "nameRef")]
    set: CardSet,

    /// The localized name of the set.
    name: String,

    /// URL to the icon of the set in `.png` format.
    #[serde(rename = "iconAbsolutePath")]
    icon_png: String,
}
