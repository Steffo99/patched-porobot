//! This module defines [CoreKeyword].

use std::collections::HashMap;
use crate::data::setbundle::keyword::CardKeyword;

/// A Legends of Runeterra [CardKeyword], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LocalizedCardKeyword {
    /// The [CardKeyword] these strings refer to.
    #[serde(rename = "nameRef")]
    pub keyword: CardKeyword,

    /// The localized name of the keyword.
    pub name: String,

    /// The description of the keyword, the text of the in-game tooltip.
    pub description: String,
}

/// How [LocalizedCardKeyword]s appear in `global.json` files.
pub type LocalizedCardKeywordVec = Vec<LocalizedCardKeyword>;
/// An index of [LocalizedCardKeyword]s, with [LocalizedCardKeyword::keyword]s as keys.
pub type LocalizedCardKeywordIndex = HashMap<CardKeyword, LocalizedCardKeyword>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, LocalizedCardKeyword>(r#"
                {
                    "description": "Inflicts damage beyond what would kill the target(s) to the enemy Nexus.",
                    "name": "Overwhelm",
                    "nameRef": "SpellOverwhelm"
                }
            "#).unwrap(),
            LocalizedCardKeyword {
                keyword: CardKeyword::SpellOverwhelm,
                name: "Overwhelm".to_string(),
                description: "Inflicts damage beyond what would kill the target(s) to the enemy Nexus.".to_string(),
            }
        );
    }
}
