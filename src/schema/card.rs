//! Module defining [Card].


use std::collections::HashMap;
use super::art::CardArt;
use super::set::CardSet;
use super::r#type::CardType;
use super::region::CardRegion;
use super::keyword::CardKeyword;
use super::rarity::CardRarity;
use super::speed::SpellSpeed;


/// A single Legends of Runeterra card as represented in the data files from [Data Dragon](https://developer.riotgames.com/docs/lor).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Card {
    /// Unique seven-character identifier of the card.
    #[serde(rename = "card_code")]
    pub code: String,

    /// Localized name of the card.
    pub name: String,

    /// The [CardType] of the card.
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
    #[deprecated = "Only for re-serialization purposes, use regions instead!"]
    #[serde(rename = "regions")]
    pub regions_localized: Vec<String>,

    /// A [Vec] of [CardArt] assets of the card.
    ///
    /// Should always contain at least an element; may sometimes contain two or more.
    ///
    /// To quickly access the first element, use [main_art].
    #[serde(rename = "assets")]
    pub art: Vec<CardArt>,

    /// Base attack of the card.
    pub attack: u64,

    /// Base cost of the card.
    pub cost: u64,

    /// Base health of the card.
    pub health: u64,

    /// [SpellSpeed] of the card.
    #[serde(rename = "spell_speed_ref")]
    pub spell_speed: SpellSpeed,
    /// Localized name of the [SpellSpeed] of the card.
    #[deprecated = "Only for re-serialization purposes, use spell_speed instead!"]
    pub spell_speed_localized: String,

    /// List of keywords of this card.
    #[serde(rename="keyword_refs")]
    pub keywords: Vec<CardKeyword>,
    /// Localized names of keywords of this card.
    #[deprecated = "Only for re-serialization purposes, use keywords instead!"]
    #[serde(rename="keywords")]
    pub keywords_localized: Vec<String>,

    /// Localized description of the card, in XML.
    pub description: String,
    /// Localized description of the card, in plain text.
    pub description_raw: String,

    /// Localized level up text of the card, in XML.
    ///
    /// If the card has no level up text, contains an empty string.
    pub levelup_description: String,
    /// Localized level up text of the card, in plain text.
    ///
    /// If the card has no level up text, contains an empty string.
    pub levelup_description_raw: String,

    /// [Codes](code) of other cards associated with this one.
    ///
    /// To access references to the cards themselves, use [associated_cards].
    #[serde(rename = "associatedCardRefs")]
    pub associated_card_codes: Vec<String>,
    /// [Names](name) of other cards associated with this one.
    ///
    /// Sometimes, it may be missing some references.
    #[deprecated = "Only for re-serialization purposes, use associated_card_codes instead!"]
    #[serde(rename = "associatedCards")]
    pub associated_card_names_localized: Vec<String>,

    /// Flavor text of the card, displayed when its image is inspected.
    pub flavor_text: String,
    /// Name of the artist who drew the card.
    pub artist_name: String,

    /// The subtypes the card has, such as `PORO`.
    pub subtypes: Vec<String>,
    /// The [CardSupertype] the card belongs to, such as `Champion`.
    pub supertype: String,
}


impl Card {
    /// Get references to the cards associated with this one, given an hashmap of all cards.
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
    use super::Card;
    use super::super::art::CardArt;
    use super::super::keyword::CardKeyword;
    use super::super::rarity::CardRarity;
    use super::super::region::CardRegion;
    use super::super::set::CardSet;
    use super::super::speed::SpellSpeed;
    use super::super::r#type::CardType;

    #[test]
    fn deserialize_card() {
        assert_eq!(
            serde_json::de::from_str(r#"
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
                #[allow(deprecated)]
                regions_localized: vec![
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
                #[allow(deprecated)]
                spell_speed_localized: String::from(""),
                keywords: vec![],
                #[allow(deprecated)]
                keywords_localized: vec![],
                description: String::from("<link=vocab.Origin><style=Vocab>Origin</style></link>: <link=card.origin><style=AssociatedCard>Agony's Embrace</style></link>.\r\nWhen I'm summoned, summon a random Husk."),
                description_raw: String::from("Origin: Agony's Embrace.\r\nWhen I'm summoned, summon a random Husk."),
                levelup_description: String::from("When you or an ally kill an allied Husk, give me its positive keywords this round and I level up."),
                levelup_description_raw: String::from("When you or an ally kill an allied Husk, give me its positive keywords this round and I level up."),
                associated_card_codes: vec![
                    String::from("06RU025T14"),
                    String::from("06RU025T6"),
                    String::from("06RU025T5"),
                ],
                associated_card_names_localized: vec![],
                flavor_text: String::from("The priestess' pupils were blown wide, and her hand trembled with nervous excitement. She was ready. This was the single moment Evelynn craved more than any other. She grinned, and slowly shed her visage. Then, as always, the screaming began."),
                artist_name: String::from("Kudos Productions"),
                subtypes: vec![],
                supertype: String::from("Champion"),
            }
        )
    }
}