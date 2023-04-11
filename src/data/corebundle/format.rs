//! Module defining structs representing localized card formats.

use crate::data::setbundle::format::CardFormat;
use std::collections::HashMap;

/// A Legends of Runeterra [CardFormat], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LocalizedCardFormat {
    /// The [CardFormat] these strings refer to.
    #[serde(rename = "nameRef")]
    pub format: CardFormat,

    /// The localized name of the keyword.
    pub name: String,

    /// URL to the icon of the set in `.png` format.
    #[serde(rename = "iconAbsolutePath")]
    pub icon_png: Option<String>,
}

/// How [LocalizedCardSet]s appear in `global.json` files.
pub type LocalizedCardFormatVec = Vec<LocalizedCardFormat>;
/// An index of [LocalizedCardSet]s, with [LocalizedCardSet::set]s as keys.
pub type LocalizedCardFormatIndex = HashMap<CardFormat, LocalizedCardFormat>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn deserialize_standard() {
        assert_eq!(
            serde_json::de::from_str::<'static, LocalizedCardFormat>(r#"
                {
                    "iconAbsolutePath": "http://dd.b.pvp.net/4_3_0/core/en_us/img/formats/queue_select_standard_toggle_active.png",
                    "name": "Standard",
                    "nameRef": "client_Formats_Standard_name"
                }
            "#).unwrap(),
            LocalizedCardFormat {
                format: CardFormat::Standard,
                name: "Standard".to_string(),
                icon_png: Some("http://dd.b.pvp.net/4_3_0/core/en_us/img/formats/queue_select_standard_toggle_active.png".to_string()),
            }
        );
    }

    #[test]
    #[rustfmt::skip]
    fn deserialize_singleton() {
        assert_eq!(
            serde_json::de::from_str::<'static, LocalizedCardFormat>(r#"
                {
                  "iconAbsolutePath": null,
                  "name": "Singleton",
                  "nameRef": "client_Deckbuilder_RulesFilters_Singleton"
                }
            "#).unwrap(),
            LocalizedCardFormat {
                format: CardFormat::Singleton,
                name: "Singleton".to_string(),
                icon_png: None,
            }
        );
    }
}
