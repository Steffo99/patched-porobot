//! This module defines [CoreSet].

use crate::schema::setbundle::CardSet;

/// A Legends of Runeterra [CardSet], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CoreSet {
    /// The [CardSet] these strings refer to.
    #[serde(rename = "nameRef")]
    pub set: CardSet,

    /// The localized name of the set.
    pub name: String,

    /// URL to the icon of the set in `.png` format.
    #[serde(rename = "iconAbsolutePath")]
    pub icon_png: String,
}


#[cfg(test)]
mod tests {
    use crate::schema::setbundle::CardSet;

    use super::CoreSet;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, CoreSet>(r#"
                {
                    "iconAbsolutePath": "http://dd.b.pvp.net/3_11_0/core/en_us/img/sets/set3_crispmip.png",
                    "name": "Call of the Mountain",
                    "nameRef": "Set3"
                }
            "#).unwrap(),
            CoreSet {
                set: CardSet::CallOfTheMountain,
                name: "Call of the Mountain".to_string(),
                icon_png: "http://dd.b.pvp.net/3_11_0/core/en_us/img/sets/set3_crispmip.png".to_string(),
            }
        );
    }
}
