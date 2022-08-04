//! This module offers ways to load official data files from Riot Games' [Data Dragon](https://developer.riotgames.com/docs/lor#data-dragon) into [Card] objects.
//!
//! Since [Card] implements both [serde::Serialize] and [serde::Deserialize], both operations can be performed manually; this module just provides shortcuts and useful utility functions.


use std::io::Read;
use std::collections::HashMap;
use crate::schema::Card;


/// Deserialize Data Dragon [Set Bundle](https://developer.riotgames.com/docs/lor#data-dragon_set-bundles) JSON data into a [Card] [Vec].
pub fn vec_from_reader<R>(r: R) -> serde_json::Result<Vec<Card>>
    where R: Read
{
    serde_json::de::from_reader::<R, Vec<Card>>(r)
}


/// Deserialize Data Dragon [Set Bundle](https://developer.riotgames.com/docs/lor#data-dragon_set-bundles) JSON data into a [Card] [HashMap], with card codes as keys.
pub fn hashmap_from_reader<R>(r: R) -> serde_json::Result<HashMap<String, Card>>
    where R: Read
{
    let vec = vec_from_reader(r)?;
    let mut hm = HashMap::<String, Card>::new();
    for card in vec {
        hm.insert(card.code.clone(), card);
    }
    Ok(hm)
}


#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::*;
    use crate::schema::*;

    const TEST_DATA: &'static str = r#"
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
            regions_localized: vec![
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
            spell_speed_localized: "Burst".to_string(),
            keywords: vec![
                CardKeyword::Burst,
            ],
            keywords_localized: vec![
                "Burst".to_string(),
            ],
            description: "Give an ally +2|+0 or +0|+3 this round.".to_string(),
            description_raw: "Give an ally +2|+0 or +0|+3 this round.".to_string(),
            levelup_description: "".to_string(),
            levelup_description_raw: "".to_string(),
            associated_card_codes: vec![],
            associated_card_names_localized: vec![],
            flavor_text: r#""Never fear change. It will question you, test your limits. It is our greatest teacher." - Karma"#.to_string(),
            artist_name: "SIXMOREVODKA".to_string(),
            subtypes: vec![],
            supertype: "".to_string()
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
            regions_localized: vec![
                "Ionia".to_string(),
            ],
            art: vec![
                CardArt {
                    card_png: "http://dd.b.pvp.net/3_11_0/set1/en_us/img/cards/01IO012T2.png".to_string(),
                    full_png: "http://dd.b.pvp.net/3_11_0/set1/en_us/img/cards/01IO012T2-full.png".to_string()
                }
            ],
            attack: 0,
            cost: 2,
            health: 0,
            spell_speed: SpellSpeed::Burst,
            spell_speed_localized: "Burst".to_string(),
            keywords: vec![
                CardKeyword::Burst,
            ],
            keywords_localized: vec![
                "Burst".to_string(),
            ],
            description: "Give an ally +0|+3 this round.".to_string(),
            description_raw: "Give an ally +0|+3 this round.".to_string(),
            levelup_description: "".to_string(),
            levelup_description_raw: "".to_string(),
            associated_card_codes: vec![],
            associated_card_names_localized: vec![],
            flavor_text: "".to_string(),
            artist_name: "SIXMOREVODKA".to_string(),
            subtypes: vec![],
            supertype: "".to_string()
        }
    }

    fn expected_hashmap() -> HashMap<String, Card> {
        let mut hm = HashMap::<String, Card>::new();
        hm.insert("01IO012".to_string(), expected_card_1());
        hm.insert("01IO012T2".to_string(), expected_card_2());
        hm
    }

    #[test]
    fn test_vec_from_reader() {
        assert_eq!(
            vec_from_reader(TEST_DATA.as_bytes()).unwrap(),
            vec![expected_card_1(), expected_card_2()]
        )
    }

    #[test]
    fn test_hashmap_from_reader() {
        assert_eq!(
            hashmap_from_reader(TEST_DATA.as_bytes()).unwrap(),
            expected_hashmap()
        )
    }
}