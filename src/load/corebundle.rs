//! This module provides ways to load official data files from Riot Games' [Core Bundles](https://developer.riotgames.com/docs/lor#data-dragon_core-bundles) into Rust structs.

use std::io::Read;
use std::collections::HashMap;
use crate::schema::corebundle::*;
use crate::schema::setbundle::*;


/// Deserialize a `globals.json` file into a [CoreGlobals] struct.
pub fn globalsjson_to_coreglobals<R>(r: R) -> serde_json::Result<CoreGlobals>
    where R: Read
{
    serde_json::de::from_reader::<R, CoreGlobals>(r)
}


/// Convert a [Vec] of [CoreVocabTerm]s into a [HashMap] of [CoreVocabTerm]s, indexed by their [CoreVocabTerm::vocabterm].
pub fn vocabtermvec_to_vocabtermhashmap(v: Vec<CoreVocabTerm>) -> HashMap<String, CoreVocabTerm> {
    let mut hm = HashMap::<String, CoreVocabTerm>::new();
    for vocabterm in v {
        hm.insert(vocabterm.vocabterm.clone(), vocabterm);
    }
    hm
}


/// Convert a [Vec] of [CoreKeyword]s into a [HashMap] of [CoreKeyword]s, indexed by their [CoreKeyword::keyword].
pub fn keywordvec_to_keywordhashmap(v: Vec<CoreKeyword>) -> HashMap<CardKeyword, CoreKeyword> {
    let mut hm = HashMap::<CardKeyword, CoreKeyword>::new();
    for keyword in v {
        hm.insert(keyword.keyword, keyword);
    }
    hm
}


/// Convert a [Vec] of [CoreRegion]s into a [HashMap] of [CoreRegion]s, indexed by their [CoreRegion::region].
pub fn regionvec_to_regionhashmap(v: Vec<CoreRegion>) -> HashMap<CardRegion, CoreRegion> {
    let mut hm = HashMap::<CardRegion, CoreRegion>::new();
    for region in v {
        hm.insert(region.region, region);
    }
    hm
}


/// Convert a [Vec] of [CoreSpellSpeed]s into a [HashMap] of [CoreSpellSpeed]s, indexed by their [CoreSpellSpeed::spell_speed].
pub fn spellspeedvec_to_spellspeedhashmap(v: Vec<CoreSpellSpeed>) -> HashMap<SpellSpeed, CoreSpellSpeed> {
    let mut hm = HashMap::<SpellSpeed, CoreSpellSpeed>::new();
    for spell_speed in v {
        hm.insert(spell_speed.spell_speed, spell_speed);
    }
    hm
}


/// Convert a [Vec] of [CoreRarity]s into a [HashMap] of [CoreRarity]s, indexed by their [CoreRarity::spell_speed].
pub fn rarityvec_to_rarityhashmap(v: Vec<CoreRarity>) -> HashMap<CardRarity, CoreRarity> {
    let mut hm = HashMap::<CardRarity, CoreRarity>::new();
    for rarity in v {
        hm.insert(rarity.rarity, rarity);
    }
    hm
}
