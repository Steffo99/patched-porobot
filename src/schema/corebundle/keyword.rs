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


#[cfg(test)]
mod tests {
    use crate::schema::setbundle::CardKeyword;

    use super::CoreKeyword;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, CoreKeyword>(r#"
                {
                    "description": "Inflicts damage beyond what would kill the target(s) to the enemy Nexus.",
                    "name": "Overwhelm",
                    "nameRef": "SpellOverwhelm"
                }
            "#).unwrap(),
            CoreKeyword {
                keyword: CardKeyword::SpellOverwhelm,
                name: "Overwhelm".to_string(),
                description: "Inflicts damage beyond what would kill the target(s) to the enemy Nexus.".to_string(),
            }
        );
    }
}
