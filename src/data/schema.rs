/// A single Legends of Runeterra card as represented in the data files from [Data Dragon](https://developer.riotgames.com/docs/lor).
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all="camelCase")]
pub struct Card {
    /// Localized names of the cards associated with this one.
    /// For some reason, might not match what is contained in `associated_card_refs`.
    pub associated_cards: Vec<String>,
    /// `card_code`s of the cards associated with this one.
    pub associated_card_refs: Vec<String>,

    /// Art assets of this card.
    pub assets: Vec<Asset>,

    /// Localized names of the regions this card belongs to.
    pub regions: Vec<String>,
    /// IDs of the regions this card belongs to.
    pub region_refs: Vec<String>,

    /// Base attack of the card.
    pub attack: i8,
    /// Base cost of the card.
    pub cost: i8,
    /// Base health of the card.
    pub health: i8,

    /// Localized description of the card, in XML.
    pub description: String,
    /// Localized description of the card, in plain text.
    pub description_raw: String,

    /// Localized level up text of the card, in XML.
    pub levelup_description: String,
    /// Localized level up text of the card, in plain text.
    pub levelup_description_raw: String,

    /// Flavor text of the card, displayed when its image is inspected.
    pub flavor_text: String,
    /// Name of the artist who drew the card.
    pub artist_name: String,

    /// Localized name of the card.
    pub name: String,
    /// Unique seven-character identifier of the card.
    pub card_code: String,

    /// List of keywords of this card, with their localized names.
    pub keywords: Vec<String>,
    /// List of keywords of this card, with their internal names.
    pub keyword_refs: Vec<String>,

    /// Localized spell speed.
    pub spell_speed: String,
    /// [SpellSpeed] of the card.
    pub spell_speed_ref: SpellSpeed,

    /// Localized rarity of the card.
    pub rarity: String,
    /// [CardRarity] of the card.
    pub rarity_ref: CardRarity,

    /// The subtypes the card has, such as `PORO`.
    pub subtypes: Vec<String>,
    /// The [CardSupertype] the card belongs to, such as `Champion`.
    pub supertype: String,

    /// If `true`, the card can be found in chests, crafted, or used in decks.
    /// If `false`, the card is not available for direct use, as it is probably created by another card.
    pub collectible: bool,

    /// The [CardSet] the card belongs to.
    pub set: String,

    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub card_type: CardType,
}


/// An art asset associated with a given card.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all="camelCase")]
pub struct Asset {
    /// URL to the card art as it is displayed in-game.
    pub game_absolute_path: String,
    /// URL to the full-size card art as it is displayed when the card is inspected.
    pub full_absolute_path: String,
}


/// Possible card types.
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CardType {
    /// A spell.
    Spell,
    /// An unit: either a minion, or a champion.
    /// Champions have their `supertype` set to `Champion`.
    Unit,
    /// An ability triggered by an unit.
    Ability,
    /// A landmark.
    Landmark,
    /// A trap or boon.
    Trap,
}


/// Possible card rarities.
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CardRarity {
    #[serde(alias = "NONE")]
    None,
    #[serde(alias = "COMMON")]
    Common,
    #[serde(alias = "RARE")]
    Rare,
    #[serde(alias = "EPIC")]
    Epic,
    #[serde(alias = "CHAMPION")]
    Champion,
}


/// Possible spell speeds.
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum SpellSpeed {
    /// Non-spell cards have this speed.
    #[serde(alias = "")]
    None,

    Slow,
    Fast,

    /// Both Focus and Burst cards have `Burst` speed; to disambiguate between the two, check for the `Focus` keyword.
    Burst,
}


/// Release sets [Card]s may belong to.
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CardSet {
    #[serde(rename = "Set1")]
    Foundations,
    #[serde(rename = "Set2")]
    RisingTides,
    #[serde(rename = "Set3")]
    CallOfTheMountain,
    #[serde(rename = "Set4")]
    EmpiresOfTheAscended,
    #[serde(rename = "Set5")]
    BeyondTheBandlewood,
    #[serde(rename = "Set6")]
    Worldwalker,
    #[serde(rename = "SetEvent")]
    Events,
}
