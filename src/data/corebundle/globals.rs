//! This module defines [LocalizedGlobalsVecs] and [LocalizedGlobalsIndexes], structs representing the data contained in the `globals.json` files.

use std::fs::File;
use std::path::Path;
use crate::data::outcomes::{LoadingError, LoadingResult};
use super::vocabterm::{LocalizedVocabTermVec, LocalizedVocabTermIndex};
use super::keyword::{LocalizedCardKeywordVec, LocalizedCardKeywordIndex};
use super::region::{LocalizedCardRegionVec, LocalizedCardRegionIndex};
use super::speed::{LocalizedSpellSpeedVec, LocalizedSpellSpeedIndex};
use super::rarity::{LocalizedCardRarityVec, LocalizedCardRarityIndex};
use super::set::{LocalizedCardSetVec, LocalizedCardSetIndex};

/// A parsed `globals.json` file from a Legends of Runeterra Core Bundle.
///
/// It contains a list of all vocabulary terms, keywords, regions, spell speeds, and rarities present in Legends of Runeterra.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LocalizedGlobalsVecs {
    #[serde(rename = "vocabTerms")]
    pub vocab_terms: LocalizedVocabTermVec,

    pub keywords: LocalizedCardKeywordVec,

    pub regions: LocalizedCardRegionVec,

    #[serde(rename = "spellSpeeds")]
    pub spell_speeds: LocalizedSpellSpeedVec,

    pub rarities: LocalizedCardRarityVec,

    pub sets: LocalizedCardSetVec,
}

/// An instance of [LocalizedGlobalsVecs] which had its own fields indexed in [HashMap]s, using the respective identifiers as map keys.
///
/// It contains a indexed list of all vocabulary terms, keywords, regions, spell speeds, and rarities present in Legends of Runeterra.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalizedGlobalsIndexes {
    pub vocab_terms: LocalizedVocabTermIndex,

    pub keywords: LocalizedCardKeywordIndex,

    pub regions: LocalizedCardRegionIndex,

    pub spell_speeds: LocalizedSpellSpeedIndex,

    pub rarities: LocalizedCardRarityIndex,

    pub sets: LocalizedCardSetIndex,
}


impl LocalizedGlobalsVecs {
    /// Load a `globals.json` file to create a [LocalizedGlobalsVecs] instance.
    pub fn load(path: &Path) -> LoadingResult<Self> {
        let file = File::open(path)
            .map_err(LoadingError::Loading)?;
        let data = serde_json::de::from_reader::<File, Self>(file)
            .map_err(LoadingError::Parsing)?;
        Ok(data)
    }
}


impl From<LocalizedGlobalsVecs> for LocalizedGlobalsIndexes {
    fn from(o: LocalizedGlobalsVecs) -> Self {
        Self {
            vocab_terms: {
                let mut hm = LocalizedVocabTermIndex::new();
                for obj in o.vocab_terms {
                    hm.insert(obj.vocabterm.clone(), obj);
                }
                hm
            },
            keywords: {
                let mut hm = LocalizedCardKeywordIndex::new();
                for obj in o.keywords {
                    hm.insert(obj.keyword, obj);
                }
                hm
            },
            regions: {
                let mut hm = LocalizedCardRegionIndex::new();
                for obj in o.regions {
                    hm.insert(obj.region, obj);
                }
                hm
            },
            spell_speeds: {
                let mut hm = LocalizedSpellSpeedIndex::new();
                for obj in o.spell_speeds {
                    hm.insert(obj.spell_speed, obj);
                }
                hm
            },
            rarities: {
                let mut hm = LocalizedCardRarityIndex::new();
                for obj in o.rarities {
                    hm.insert(obj.rarity, obj);
                }
                hm
            },
            sets: {
                let mut hm = LocalizedCardSetIndex::new();
                for obj in o.sets {
                    hm.insert(obj.set, obj);
                }
                hm
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::setbundle::keyword::CardKeyword;
    use crate::data::setbundle::rarity::CardRarity;
    use crate::data::setbundle::region::CardRegion;
    use crate::data::setbundle::set::CardSet;
    use crate::data::setbundle::speed::SpellSpeed;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, LocalizedGlobalsVecs>(r#"
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
            LocalizedGlobalsVecs {
                vocab_terms: vec![
                    vocabterm::LocalizedVocabTerm {
                        vocabterm: "Allegiance".to_string(),
                        name: "Allegiance".to_string(),
                        description: "When you summon this, it gets its allegiance bonus if the top card of your deck matches its region.".to_string(),
                    }
                ],
                keywords: vec![
                    keyword::LocalizedCardKeyword {
                        keyword: CardKeyword::SpellOverwhelm,
                        name: "Overwhelm".to_string(),
                        description: "Inflicts damage beyond what would kill the target(s) to the enemy Nexus.".to_string(),
                    }
                ],
                regions: vec![
                    region::LocalizedCardRegion {
                        region: CardRegion::Noxus,
                        name: "Noxus".to_string(),
                        abbreviation: "NX".to_string(),
                        icon_png: "http://dd.b.pvp.net/3_11_0/core/en_us/img/regions/icon-noxus.png".to_string(),
                    }
                ],
                spell_speeds: vec![
                    speed::LocalizedSpellSpeed {
                        spell_speed: SpellSpeed::Slow,
                        name: "Slow".to_string(),
                    }
                ],
                rarities: vec![
                    rarity::LocalizedCardRarity {
                        rarity: CardRarity::Common,
                        name: "COMMON".to_string(),
                    }
                ],
                sets: vec![
                    set::LocalizedCardSet {
                        set: CardSet::CallOfTheMountain,
                        name: "Call of the Mountain".to_string(),
                        icon_png: "http://dd.b.pvp.net/3_11_0/core/en_us/img/sets/set3_crispmip.png".to_string(),
                    }
                ]
            }
        )
    }
}