//! This module defines [CoreRarity].

use crate::schema::setbundle::CardRarity;

/// A Legends of Runeterra [CardRarity], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CoreRarity {
    /// The [CardRarity] these strings refer to.
    #[serde(rename = "nameRef")]
    pub rarity: CardRarity,

    /// The localized name of the rarity.
    pub name: String,
}


#[cfg(test)]
mod tests {
    use crate::schema::setbundle::CardRarity;

    use super::CoreRarity;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, CoreRarity>(r#"
                {
                    "name": "COMMON",
                    "nameRef": "Common"
                }
            "#).unwrap(),
            CoreRarity {
                rarity: CardRarity::Common,
                name: "COMMON".to_string(),
            }
        );
    }
}
