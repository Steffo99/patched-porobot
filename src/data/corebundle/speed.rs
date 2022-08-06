//! Module defining structs representing localized spell speeds.

use std::collections::HashMap;
use crate::data::setbundle::speed::SpellSpeed;

/// A Legends of Runeterra [SpellSpeed], and its associated localization.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LocalizedSpellSpeed {
    /// The [SpellSpeed] these strings refer to.
    #[serde(rename = "nameRef")]
    pub spell_speed: SpellSpeed,

    /// The localized name of the spell speed.
    pub name: String,
}

/// How [LocalizedSpellSpeed]s appear in `global.json` files.
pub type LocalizedSpellSpeedVec = Vec<LocalizedSpellSpeed>;
/// An index of [LocalizedSpellSpeed]s, with [LocalizedSpellSpeed::spell_speed]s as keys.
pub type LocalizedSpellSpeedIndex = HashMap<SpellSpeed, LocalizedSpellSpeed>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, LocalizedSpellSpeed>(r#"
                {
                    "name": "Slow",
                    "nameRef": "Slow"
                }
            "#).unwrap(),
            LocalizedSpellSpeed {
                spell_speed: SpellSpeed::Slow,
                name: "Slow".to_string(),
            }
        );
    }
}
