//! Module defining [Card].


use std::collections::HashMap;
use super::*;

/// A single Legends of Runeterra card as represented in a `set.json` file.
///
/// The information is represented in a developer-friendly manner, but it can be serialized and deserialized via [serde] in the exact same format used in Data Dragon.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Card {
    /// Unique seven-character identifier of the card.
    #[serde(rename = "cardCode")]
    pub code: String,

    /// Localized name of the card.
    pub name: String,

    /// The [CardType] of the card.
    ///
    /// The `r#` prefix is required by the Rust syntax, since `type` is a reserved keyword.
    #[serde(rename = "type")]
    pub r#type: CardType,

    /// The [CardSet] the card belongs to.
    pub set: CardSet,

    /// [CardRarity] of the card.
    #[serde(rename = "rarityRef")]
    pub rarity: CardRarity,

    /// If `true`, the card can be found in chests, crafted, or used in decks.
    /// If `false`, the card is not available for direct use, as it is probably created by another card.
    pub collectible: bool,

    /// Regions this card belongs to.
    #[serde(rename = "regionRefs")]
    pub regions: Vec<CardRegion>,

    /// Localized names of the regions this card belongs to.
    ///
    /// For serialization purposes only, use the [method with the same name](Card::localized_regions()] instead!
    #[serde(rename = "regions")]
    pub(crate) localized_regions: Vec<String>,

    /// A [Vec] of [CardArt] assets of the card.
    ///
    /// Should always contain at least an element; may sometimes contain two or more.
    ///
    /// To quickly access the first element, use [Card::main_art].
    #[serde(rename = "assets")]
    pub art: Vec<CardArt>,

    /// Base attack of the card.
    ///
    /// Cards with no attack, such as [CardType::Spell]s, have a value of `0` attack.
    pub attack: u64,

    /// Base mana cost of the card.
    ///
    /// Cards with no health, such as [CardType::Ability]s, have a value of `0` cost.
    pub cost: u64,

    /// Base health of the card.
    ///
    /// Cards with no health, such as [CardType::Spell]s, have a value of `0` health.
    pub health: u64,

    /// [SpellSpeed] of the card.
    #[serde(rename = "spellSpeedRef")]
    pub spell_speed: SpellSpeed,

    /// Localized name of the [SpellSpeed] of the card.
    ///
    /// For serialization purposes only, use the [method with the same name](Card::localized_spell_speed()] instead!
    #[serde(rename = "spellSpeed")]
    pub(crate) localized_spell_speed: String,

    /// [Vec] of [CardKeyword]s of the card.
    #[serde(rename = "keywordRefs")]
    pub keywords: Vec<CardKeyword>,

    /// [Vec] of localized names of [CardKeyword]s of the card.
    ///
    /// For serialization purposes only, use the [method with the same name](Card::localized_keywords()] instead!
    #[serde(rename = "keywords")]
    pub(crate) localized_keywords: Vec<String>,

    /// Localized description of the card, in pseudo-XML.
    #[serde(rename = "description")]
    pub localized_description_xml: String,

    /// Localized description of the card, in plain text.
    #[serde(rename = "descriptionRaw")]
    pub localized_description_text: String,

    /// Localized level up text of the card, in pseudo-XML.
    ///
    /// If the card has no level up text, contains an empty string.
    #[serde(rename = "levelupDescription")]
    pub localized_levelup_xml: String,

    /// Localized level up text of the card, in plain text.
    ///
    /// If the card has no level up text, contains an empty string.
    #[serde(rename = "levelupDescriptionRaw")]
    pub localized_levelup_text: String,

    /// [Vec] with [Card::code]s of other cards associated with this one.
    ///
    /// To access references to the cards themselves, use [Card::associated_cards].
    #[serde(rename = "associatedCardRefs")]
    pub associated_card_codes: Vec<String>,

    /// [Vec] with [Card::name]s of other cards associated with this one.
    ///
    /// Sometimes, it may be missing some references.
    #[serde(rename = "associatedCards")]
    pub(crate) associated_card_names_localized: Vec<String>,

    /// Flavor text of the card, displayed when its image is inspected.
    #[serde(rename = "flavorText")]
    pub localized_flavor_text: String,

    /// Name of the artist who drew the card.
    #[serde(rename = "artistName")]
    pub artist_name: String,

    /// The subtypes the card has, such as `"PORO"`.
    ///
    /// Beware of Riot's inconsistent capitalization!
    ///
    /// TODO: Make this a enum.
    pub subtypes: Vec<String>,

    /// The supertype the card belongs to, such as `"Champion"`.
    ///
    /// Beware of Riot's inconsistent capitalization!
    ///
    /// TODO: Make this a enum.
    pub supertype: String,
}


impl Card {
    /// Get references to the cards associated with this one, given an [HashMap] of cards indexed by code.
    pub fn associated_cards<'c, 'hm: 'c>(&'c self, hashmap: &'hm HashMap<String, Card>) -> impl Iterator<Item=Option<&'hm Card>> + 'c {
        self.associated_card_codes.iter().map(|r| hashmap.get(r))
    }

    /// Get a reference to the first [CardArt] of the card.
    ///
    /// # Panics
    ///
    /// If the card has no associated [CardArt].
    pub fn main_art(&self) -> &CardArt {
        self.art.get(0).expect("card to have at least one art asset")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_card() {
        assert_eq!(
            serde_json::de::from_str::<'static, Card>(r#"
            {
                "associatedCards": [],
                "associatedCardRefs": [
                  "06RU025T14",
                  "06RU025T6",
                  "06RU025T5"
                ],
                "assets": [
                    {
                        "gameAbsolutePath": "http://dd.b.pvp.net/3_11_0/set6/en_us/img/cards/06RU025.png",
                        "fullAbsolutePath": "http://dd.b.pvp.net/3_11_0/set6/en_us/img/cards/06RU025-full.png"
                    }
                ],
                "regions": [
                    "Runeterra"
                ],
                "regionRefs": [
                    "Runeterra"
                ],
                "attack": 0,
                "cost": 4,
                "health": 5,
                "description": "<link=vocab.Origin><style=Vocab>Origin</style></link>: <link=card.origin><style=AssociatedCard>Agony's Embrace</style></link>.\r\nWhen I'm summoned, summon a random Husk.",
                "descriptionRaw": "Origin: Agony's Embrace.\r\nWhen I'm summoned, summon a random Husk.",
                "levelupDescription": "When you or an ally kill an allied Husk, give me its positive keywords this round and I level up.",
                "levelupDescriptionRaw": "When you or an ally kill an allied Husk, give me its positive keywords this round and I level up.",
                "flavorText": "The priestess' pupils were blown wide, and her hand trembled with nervous excitement. She was ready. This was the single moment Evelynn craved more than any other. She grinned, and slowly shed her visage. Then, as always, the screaming began.",
                "artistName": "Kudos Productions",
                "name": "Evelynn",
                "cardCode": "06RU025",
                "keywords": [],
                "keywordRefs": [],
                "spellSpeed": "",
                "spellSpeedRef": "",
                "rarity": "Champion",
                "rarityRef": "Champion",
                "subtypes": [],
                "supertype": "Champion",
                "type": "Unit",
                "collectible": true,
                "set": "Set6"
            }
            "#).unwrap(),
            Card {
                code: String::from("06RU025"),
                name: String::from("Evelynn"),
                r#type: CardType::Unit,
                set: CardSet::Worldwalker,
                rarity: CardRarity::Champion,
                collectible: true,
                regions: vec![
                    CardRegion::Runeterra
                ],
                localized_regions: vec![
                    String::from("Runeterra")
                ],
                art: vec![
                    CardArt {
                        card_png: String::from("http://dd.b.pvp.net/3_11_0/set6/en_us/img/cards/06RU025.png"),
                        full_png: String::from("http://dd.b.pvp.net/3_11_0/set6/en_us/img/cards/06RU025-full.png"),
                    }
                ],
                attack: 0u64,
                cost: 4u64,
                health: 5u64,
                spell_speed: SpellSpeed::None,
                localized_spell_speed: String::from(""),
                keywords: vec![],
                localized_keywords: vec![],
                localized_description_xml: String::from("<link=vocab.Origin><style=Vocab>Origin</style></link>: <link=card.origin><style=AssociatedCard>Agony's Embrace</style></link>.\r\nWhen I'm summoned, summon a random Husk."),
                localized_description_text: String::from("Origin: Agony's Embrace.\r\nWhen I'm summoned, summon a random Husk."),
                localized_levelup_xml: String::from("When you or an ally kill an allied Husk, give me its positive keywords this round and I level up."),
                localized_levelup_text: String::from("When you or an ally kill an allied Husk, give me its positive keywords this round and I level up."),
                associated_card_codes: vec![
                    String::from("06RU025T14"),
                    String::from("06RU025T6"),
                    String::from("06RU025T5"),
                ],
                associated_card_names_localized: vec![],
                localized_flavor_text: String::from("The priestess' pupils were blown wide, and her hand trembled with nervous excitement. She was ready. This was the single moment Evelynn craved more than any other. She grinned, and slowly shed her visage. Then, as always, the screaming began."),
                artist_name: String::from("Kudos Productions"),
                subtypes: vec![],
                supertype: String::from("Champion"),
            }
        )
    }
}