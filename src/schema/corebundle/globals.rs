use super::keyword::CoreKeyword;
use super::rarity::CoreRarity;
use super::region::CoreRegion;
use super::speed::CoreSpellSpeed;
use super::vocabterm::CoreVocabTerm;

/// A complete `globals.json` file.
///
/// It contains a list of all vocabulary terms, [CardKeyword]s, [CardRegion]s, [SpellSpeed]s, and [CardRarity]s present in the game.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CoreGlobals {
    /// A [Vec] of all [CoreVocabTerm]s in the game.
    #[serde(rename = "vocabTerms")]
    pub vocab_terms: Vec<CoreVocabTerm>,

    /// A [Vec] of all [CoreKeyword]s in the game.
    pub keywords: Vec<CoreKeyword>,

    /// A [Vec] of all [CoreRegion]s in the game.
    pub regions: Vec<CoreRegion>,

    /// A [Vec] of all [CoreSpellSpeed]s in the game.
    #[serde(rename = "spellSpeeds")]
    pub spell_speeds: Vec<CoreSpellSpeed>,

    /// A [Vec] of all [CoreRarity]s in the game.
    pub rarities: Vec<CoreRarity>,
}


#[cfg(test)]
mod tests {
    use crate::schema::corebundle::*;
    use crate::schema::setbundle::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, CoreGlobals>(r#"
                {
                    "vocabTerms": [
                        {
                            "description": "When you summon this, it gets its allegiance bonus if the top card of your deck matches its region.",
                            "name": "Allegiance",
                            "nameRef": "Allegiance"
                        }
                    ],
                    "keywords": [
                        {
                            "description": "Inflicts damage beyond what would kill the target(s) to the enemy Nexus.",
                            "name": "Overwhelm",
                            "nameRef": "SpellOverwhelm"
                        }
                    ],
                    "regions": [
                        {
                            "abbreviation": "NX",
                            "iconAbsolutePath": "http://dd.b.pvp.net/3_11_0/core/en_us/img/regions/icon-noxus.png",
                            "name": "Noxus",
                            "nameRef": "Noxus"
                        }
                    ],
                    "spellSpeeds": [
                        {
                            "name": "Slow",
                            "nameRef": "Slow"
                        }
                    ],
                    "rarities": [
                        {
                            "name": "COMMON",
                            "nameRef": "Common"
                        }
                    ],
                    "sets": [
                        {
                            "iconAbsolutePath": "http://dd.b.pvp.net/3_11_0/core/en_us/img/sets/set3_crispmip.png",
                            "name": "Call of the Mountain",
                            "nameRef": "Set3"
                        }
                    ]
                }
            "#).unwrap(),
            CoreGlobals {
                vocab_terms: vec![
                    CoreVocabTerm {
                        vocabterm: "Allegiance".to_string(),
                        name: "Allegiance".to_string(),
                        description: "When you summon this, it gets its allegiance bonus if the top card of your deck matches its region.".to_string(),
                    }
                ],
                keywords: vec![
                    CoreKeyword {
                        keyword: CardKeyword::SpellOverwhelm,
                        name: "Overwhelm".to_string(),
                        description: "Inflicts damage beyond what would kill the target(s) to the enemy Nexus.".to_string(),
                    }
                ],
                regions: vec![
                    CoreRegion {
                        region: CardRegion::Noxus,
                        name: "Noxus".to_string(),
                        abbreviation: "NX".to_string(),
                        icon_png: "http://dd.b.pvp.net/3_11_0/core/en_us/img/regions/icon-noxus.png".to_string(),
                    }
                ],
                spell_speeds: vec![
                    CoreSpellSpeed {
                        spell_speed: SpellSpeed::Slow,
                        name: "Slow".to_string(),
                    }
                ],
                rarities: vec![
                    CoreRarity {
                        rarity: CardRarity::Common,
                        name: "COMMON".to_string(),
                    }
                ],
            }
        )
    }
}