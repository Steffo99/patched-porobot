//! This module defines [CoreSpellSpeed].

use crate::schema::setbundle::SpellSpeed;

/// A Legends of Runeterra [SpellSpeed], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CoreSpellSpeed {
    /// The [SpellSpeed] these strings refer to.
    #[serde(rename = "nameRef")]
    pub spell_speed: SpellSpeed,

    /// The localized name of the spell speed.
    pub name: String,
}


#[cfg(test)]
mod tests {
    use crate::schema::setbundle::SpellSpeed;

    use super::CoreSpellSpeed;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, CoreSpellSpeed>(r#"
                {
                    "name": "Slow",
                    "nameRef": "Slow"
                }
            "#).unwrap(),
            CoreSpellSpeed {
                spell_speed: SpellSpeed::Slow,
                name: "Slow".to_string(),
            }
        );
    }
}
