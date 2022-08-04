//! This module defines [CoreKeyword].

use crate::schema::setbundle::CardKeyword;

/// A Legends of Runeterra [CardKeyword], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CoreKeyword {
    /// The [CardKeyword] these strings refer to.
    #[serde(rename = "nameRef")]
    pub keyword: CardKeyword,

    /// The localized name of the keyword.
    pub name: String,

    /// The description of the keyword, the text of the in-game tooltip.
    pub description: String,
}
