//! Module defining structs representing localized card sets.

use crate::data::setbundle::set::CardSet;
use std::collections::HashMap;

/// A Legends of Runeterra [CardSet], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LocalizedCardSet {
    /// The [CardSet] these strings refer to.
    #[serde(rename = "nameRef")]
    pub set: CardSet,

    /// The localized name of the set.
    pub name: String,

    /// URL to the icon of the set in `.png` format.
    #[serde(rename = "iconAbsolutePath")]
    pub icon_png: String,
}

/// How [LocalizedCardSet]s appear in `global.json` files.
pub type LocalizedCardSetVec = Vec<LocalizedCardSet>;
/// An index of [LocalizedCardSet]s, with [LocalizedCardSet::set]s as keys.
pub type LocalizedCardSetIndex = HashMap<CardSet, LocalizedCardSet>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, LocalizedCardSet>(r#"
                {
                    "iconAbsolutePath": "http://dd.b.pvp.net/3_11_0/core/en_us/img/sets/set3_crispmip.png",
                    "name": "Call of the Mountain",
                    "nameRef": "Set3"
                }
            "#).unwrap(),
            LocalizedCardSet {
                set: CardSet::CallOfTheMountain,
                name: "Call of the Mountain".to_string(),
                icon_png: "http://dd.b.pvp.net/3_11_0/core/en_us/img/sets/set3_crispmip.png".to_string(),
            }
        );
    }
}
