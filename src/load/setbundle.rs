//! This module provides ways to load official data files from Riot Games' [Set Bundles](https://developer.riotgames.com/docs/lor#data-dragon_set-bundles) into Rust structs.

use std::collections::HashMap;
use std::io::Read;

use crate::schema::setbundle::*;

/// Deserialize a `set.json` file into a [Vec] of [Card]s.
pub fn setjson_to_cardvec<R>(r: R) -> serde_json::Result<Vec<Card>>
    where R: Read
{
    serde_json::de::from_reader::<R, Vec<Card>>(r)
}


/// Convert a [Vec] of [Card]s (probably from [setjson_to_vec]) into a [HashMap] of [Card]s, indexed by their [Card::code].
pub fn cardvec_to_cardhashmap(v: Vec<Card>) -> HashMap<String, Card> {
    let mut hm = HashMap::<String, Card>::new();
    for card in v {
        hm.insert(card.code.clone(), card);
    }
    hm
}


#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use std::collections::HashMap;

    use crate::schema::setbundle::*;

    use super::cardvec_to_cardhashmap;
    use super::setjson_to_cardvec;

    const TEST_SETJSON: &str = r#"
        [
            {
                "associatedCards": [],
                "associatedCardRefs": [],
                "assets": [
                    {
                        "gameAbsolutePath": "http://dd.b.pvp.net/3_11_0/set1/en_us/img/cards/01IO012.png",
                        "fullAbsolutePath": "http://dd.b.pvp.net/3_11_0/set1/en_us/img/cards/01IO012-full.png"
                    }
                ],
                "regions": [
                    "Ionia"
                ],
                "regionRefs": [
                    "Ionia"
                ],
                "attack": 0,
                "cost": 2,
                "health": 0,
                "description": "Give an ally +2|+0 or +0|+3 this round.",
                "descriptionRaw": "Give an ally +2|+0 or +0|+3 this round.",
                "levelupDescription": "",
                "levelupDescriptionRaw": "",
                "flavorText": "\"Never fear change. It will question you, test your limits. It is our greatest teacher.\" - Karma",
                "artistName": "SIXMOREVODKA",
                "name": "Twin Disciplines",
                "cardCode": "01IO012",
                "keywords": [
                    "Burst"
                ],
                "keywordRefs": [
                    "Burst"
                ],
                "spellSpeed": "Burst",
                "spellSpeedRef": "Burst",
                "rarity": "COMMON",
                "rarityRef": "Common",
                "subtypes": [],
                "supertype": "",
                "type": "Spell",
                "collectible": true,
                "set": "Set1"
            },
            {
                "associatedCards": [],
                "associatedCardRefs": [],
                "assets": [
                    {
                        "gameAbsolutePath": "http://dd.b.pvp.net/3_11_0/set1/en_us/img/cards/01IO012T2.png",
                        "fullAbsolutePath": "http://dd.b.pvp.net/3_11_0/set1/en_us/img/cards/01IO012T2-full.png"
                    }
                ],
                "regions": [
                    "Ionia"
                ],
                "regionRefs": [
                    "Ionia"
                ],
                "attack": 0,
                "cost": 2,
                "health": 0,
                "description": "Give an ally +0|+3 this round.",
                "descriptionRaw": "Give an ally +0|+3 this round.",
                "levelupDescription": "",
                "levelupDescriptionRaw": "",
                "flavorText": "",
                "artistName": "SIXMOREVODKA",
                "name": "Discipline of Fortitude",
                "cardCode": "01IO012T2",
                "keywords": [
                    "Burst"
                ],
                "keywordRefs": [
                    "Burst"
                ],
                "spellSpeed": "Burst",
                "spellSpeedRef": "Burst",
                "rarity": "None",
                "rarityRef": "None",
                "subtypes": [],
                "supertype": "",
                "type": "Spell",
                "collectible": false,
                "set": "Set1"
            }
        ]
    "#;

    fn expected_card_1() -> Card {
        Card {
            code: "01IO012".to_string(),
            name: "Twin Disciplines".to_string(),
            r#type: CardType::Spell,
            set: CardSet::Foundations,
            rarity: CardRarity::Common,
            collectible: true,
            regions: vec![
                CardRegion::Ionia,
            ],
            localized_regions: vec![
                "Ionia".to_string(),
            ],
            art: vec![
                CardArt {
                    card_png: "http://dd.b.pvp.net/3_11_0/set1/en_us/img/cards/01IO012.png".to_string(),
                    full_png: "http://dd.b.pvp.net/3_11_0/set1/en_us/img/cards/01IO012-full.png".to_string(),
                }
            ],
            attack: 0,
            cost: 2,
            health: 0,
            spell_speed: SpellSpeed::Burst,
            localized_spell_speed: "Burst".to_string(),
            keywords: vec![
                CardKeyword::Burst,
            ],
            localized_keywords: vec![
                "Burst".to_string(),
            ],
            localized_description_xml: "Give an ally +2|+0 or +0|+3 this round.".to_string(),
            localized_description_text: "Give an ally +2|+0 or +0|+3 this round.".to_string(),
            localized_levelup_xml: "".to_string(),
            localized_levelup_text: "".to_string(),
            associated_card_codes: vec![],
            associated_card_names_localized: vec![],
            localized_flavor_text: r#""Never fear change. It will question you, test your limits. It is our greatest teacher." - Karma"#.to_string(),
            artist_name: "SIXMOREVODKA".to_string(),
            subtypes: vec![],
            supertype: "".to_string(),
        }
    }

    fn expected_card_2() -> Card {
        Card {
            code: "01IO012T2".to_string(),
            name: "Discipline of Fortitude".to_string(),
            r#type: CardType::Spell,
            set: CardSet::Foundations,
            rarity: CardRarity::None,
            collectible: false,
            regions: vec![
                CardRegion::Ionia,
            ],
            localized_regions: vec![
                "Ionia".to_string(),
            ],
            art: vec![
                CardArt {
                    card_png: "http://dd.b.pvp.net/3_11_0/set1/en_us/img/cards/01IO012T2.png".to_string(),
                    full_png: "http://dd.b.pvp.net/3_11_0/set1/en_us/img/cards/01IO012T2-full.png".to_string(),
                }
            ],
            attack: 0,
            cost: 2,
            health: 0,
            spell_speed: SpellSpeed::Burst,
            localized_spell_speed: "Burst".to_string(),
            keywords: vec![
                CardKeyword::Burst,
            ],
            localized_keywords: vec![
                "Burst".to_string(),
            ],
            localized_description_xml: "Give an ally +0|+3 this round.".to_string(),
            localized_description_text: "Give an ally +0|+3 this round.".to_string(),
            localized_levelup_xml: "".to_string(),
            localized_levelup_text: "".to_string(),
            associated_card_codes: vec![],
            associated_card_names_localized: vec![],
            localized_flavor_text: "".to_string(),
            artist_name: "SIXMOREVODKA".to_string(),
            subtypes: vec![],
            supertype: "".to_string(),
        }
    }

    fn expected_vec() -> Vec<Card> {
        vec![expected_card_1(), expected_card_2()]
    }

    fn expected_hashmap() -> HashMap<String, Card> {
        let mut hm = HashMap::<String, Card>::new();
        hm.insert("01IO012".to_string(), expected_card_1());
        hm.insert("01IO012T2".to_string(), expected_card_2());
        hm
    }

    #[test]
    fn test_setjson_to_cardvec() {
        assert_eq!(
            setjson_to_cardvec(TEST_SETJSON.as_bytes()).unwrap(),
            expected_vec()
        )
    }

    #[test]
    fn test_cardvec_to_cardhashmap() {
        assert_eq!(
            cardvec_to_cardhashmap(expected_vec()),
            expected_hashmap()
        )
    }
}