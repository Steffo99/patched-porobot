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
