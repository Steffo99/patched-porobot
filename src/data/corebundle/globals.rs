//! Module defining structs representing data contained in `globals.json` files.

use super::keyword::{LocalizedCardKeywordIndex, LocalizedCardKeywordVec};
use super::rarity::{LocalizedCardRarityIndex, LocalizedCardRarityVec};
use super::region::{LocalizedCardRegionIndex, LocalizedCardRegionVec};
use super::set::{LocalizedCardSetIndex, LocalizedCardSetVec};
use super::speed::{LocalizedSpellSpeedIndex, LocalizedSpellSpeedVec};
use super::vocabterm::{LocalizedVocabTermIndex, LocalizedVocabTermVec};
use crate::data::anybundle::outcomes::{LoadingError, LoadingResult};
use std::fs::File;
use std::path::Path;

/// A parsed `globals.json` file from a [Data Dragon] [Core Bundle].
///
/// It contains [Vec]s of all vocabulary terms, keywords, regions, spell speeds, and rarities present in the game.
///
/// [Data Dragon]: https://developer.riotgames.com/docs/lor#data-dragon
/// [Core Bundle]: https://developer.riotgames.com/docs/lor#data-dragon_core-bundles
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LocalizedGlobalsVecs {
    /// Vocabulary terms.
    #[serde(rename = "vocabTerms")]
    pub vocab_terms: LocalizedVocabTermVec,

    /// Card keywords.
    pub keywords: LocalizedCardKeywordVec,

    /// Card regions.
    pub regions: LocalizedCardRegionVec,

    /// Spell speeds.
    #[serde(rename = "spellSpeeds")]
    pub spell_speeds: LocalizedSpellSpeedVec,

    /// Card rarities.
    pub rarities: LocalizedCardRarityVec,

    /// Card sets.
    pub sets: LocalizedCardSetVec,
}

/// A parsed and indexed `globals.json` file from a [Data Dragon] [Core Bundle].
///
/// It is an instance of [LocalizedGlobalsVecs] which had its own fields indexed in [std::collections::HashMap]s, using the respective identifiers as map keys.
///
/// It contains indexes of all vocabulary terms, keywords, regions, spell speeds, and rarities present in the game.
///
/// [Data Dragon]: https://developer.riotgames.com/docs/lor#data-dragon
/// [Core Bundle]: https://developer.riotgames.com/docs/lor#data-dragon_core-bundles
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalizedGlobalsIndexes {
    /// Vocabulary terms.
    pub vocab_terms: LocalizedVocabTermIndex,

    /// Card keywords.
    pub keywords: LocalizedCardKeywordIndex,

    /// Card regions.
    pub regions: LocalizedCardRegionIndex,

    /// Spell speeds.
    pub spell_speeds: LocalizedSpellSpeedIndex,

    /// Card rarities.
    pub rarities: LocalizedCardRarityIndex,

    /// Card sets.
    pub sets: LocalizedCardSetIndex,
}

impl LocalizedGlobalsVecs {
    /// Load a `globals.json` file to create a [LocalizedGlobalsVecs] instance.
    pub fn load(path: &Path) -> LoadingResult<Self> {
        let file = File::open(path).map_err(LoadingError::OpeningFile)?;
        let data =
            serde_json::de::from_reader::<File, Self>(file).map_err(LoadingError::Deserializing)?;
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
    use super::LocalizedGlobalsVecs;
    use crate::data::corebundle::keyword::LocalizedCardKeyword;
    use crate::data::corebundle::rarity::LocalizedCardRarity;
    use crate::data::corebundle::region::LocalizedCardRegion;
    use crate::data::corebundle::set::LocalizedCardSet;
    use crate::data::corebundle::speed::LocalizedSpellSpeed;
    use crate::data::corebundle::vocabterm::LocalizedVocabTerm;
    use crate::data::setbundle::keyword::CardKeyword;
    use crate::data::setbundle::rarity::CardRarity;
    use crate::data::setbundle::region::CardRegion;
    use crate::data::setbundle::set::CardSet;
    use crate::data::setbundle::speed::SpellSpeed;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, LocalizedGlobalsVecs>(
                r#"
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
            "#
            )
            .unwrap(),
            LocalizedGlobalsVecs { vocab_terms: vec![LocalizedVocabTerm { vocabterm: "Allegiance".to_string(), name: "Allegiance".to_string(), description: "When you summon this, it gets its allegiance bonus if the top card of your deck matches its region.".to_string() }], keywords: vec![LocalizedCardKeyword { keyword: CardKeyword::SpellOverwhelm, name: "Overwhelm".to_string(), description: "Inflicts damage beyond what would kill the target(s) to the enemy Nexus.".to_string() }], regions: vec![LocalizedCardRegion { region: CardRegion::Noxus, name: "Noxus".to_string(), abbreviation: "NX".to_string(), icon_png: "http://dd.b.pvp.net/3_11_0/core/en_us/img/regions/icon-noxus.png".to_string() }], spell_speeds: vec![LocalizedSpellSpeed { spell_speed: SpellSpeed::Slow, name: "Slow".to_string() }], rarities: vec![LocalizedCardRarity { rarity: CardRarity::Common, name: "COMMON".to_string() }], sets: vec![LocalizedCardSet { set: CardSet::CallOfTheMountain, name: "Call of the Mountain".to_string(), icon_png: "http://dd.b.pvp.net/3_11_0/core/en_us/img/sets/set3_crispmip.png".to_string() }] }
        )
    }
}
