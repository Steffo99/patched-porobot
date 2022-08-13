//! Module defining structs representing localized card regions.

use crate::data::setbundle::region::CardRegion;
use std::collections::HashMap;

/// A Legends of Runeterra [CardRegion], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LocalizedCardRegion {
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

/// How [LocalizedCardRegion]s appear in `global.json` files.
pub type LocalizedCardRegionVec = Vec<LocalizedCardRegion>;
/// An index of [LocalizedCardRegion]s, with [LocalizedCardRegion::region]s as keys.
pub type LocalizedCardRegionIndex = HashMap<CardRegion, LocalizedCardRegion>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, LocalizedCardRegion>(
                r#"
                {
                    "abbreviation": "NX",
                    "iconAbsolutePath": "http://dd.b.pvp.net/3_11_0/core/en_us/img/regions/icon-noxus.png",
                    "name": "Noxus",
                    "nameRef": "Noxus"
                }
            "#
            )
            .unwrap(),
            LocalizedCardRegion {
                region: CardRegion::Noxus,
                name: "Noxus".to_string(),
                abbreviation: "NX".to_string(),
                icon_png: "http://dd.b.pvp.net/3_11_0/core/en_us/img/regions/icon-noxus.png".to_string()
            }
        );
    }
}
