/// A single Legends of Runeterra card as represented in the data files from [Data Dragon](https://developer.riotgames.com/docs/lor).
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct Card {
    /// Localized names of the cards associated with this one.
    /// For some reason, might not match what is contained in `associated_card_refs`.
    associated_cards: Vec<String>,
    /// `card_code`s of the cards associated with this one.
    associated_card_refs: Vec<String>,

    /// Art assets of this card.
    assets: Vec<Asset>,

    /// Localized names of the regions this card belongs to.
    regions: Vec<String>,
    /// IDs of the regions this card belongs to.
    region_refs: Vec<String>,

    /// Base attack of the card.
    attack: i8,
    /// Base cost of the card.
    cost: i8,
    /// Base health of the card.
    health: i8,

    /// Localized description of the card, in XML.
    description: String,
    /// Localized description of the card, in plain text.
    description_raw: String,

    /// Localized level up text of the card, in XML.
    levelup_description: String,
    /// Localized level up text of the card, in plain text.
    levelup_description_raw: String,

    /// Flavor text of the card, displayed when its image is inspected.
    flavor_text: String,
    /// Name of the artist who drew the card.
    artist_name: String,

    /// Localized name of the card.
    name: String,
    /// Unique seven-character identifier of the card.
    card_code: [char; 7],

    /// List of keywords of this card, with their localized names.
    keywords: Vec<String>,
    /// List of keywords of this card, with their internal names.
    keyword_refs: Vec<String>,

    /// Localized spell speed.
    spell_speed: String,
    /// [SpellSpeed] of the card.
    spell_speed_ref: SpellSpeed,

    /// Localized rarity of the card.
    rarity: String,
    /// [CardRarity] of the card.
    rarity_ref: CardRarity,

    /// The subtypes the card has, such as `PORO`.
    subtypes: Vec<String>,
    /// The [CardSupertype] the card belongs to, such as `Champion`.
    supertype: CardSupertype,

    /// If `true`, the card can be found in chests, crafted, or used in decks.
    /// If `false`, the card is not available for direct use, as it is probably created by another card.
    collectible: bool,

    /// The [CardSet] the card belongs to.
    set: String,

    #[serde(rename(serialize = "type", deserialize = "type"))]
    card_type: CardType,
}


/// An art asset associated with a given card.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct Asset {
    /// URL to the card art as it is displayed in-game.
    game_absolute_path: String,
    /// URL to the full-size card art as it is displayed when the card is inspected.
    full_absolute_path: String,
}


/// Possible card types.
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum CardType {
    /// A spell.
    Spell,
    /// An unit: either a minion, or a champion.
    /// Champions have their `supertype` set to `Champion`.
    Unit,
    /// An ability triggered by an unit.
    Ability,
    /// A landmark.
    Landmark,
}


/// Possible card supertypes.
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum CardSupertype {
    #[serde(alias = "")]
    None,

    Champion,
}


/// Possible card rarities.
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum CardRarity {
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
#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum SpellSpeed {
    /// Non-spell cards have this speed.
    #[serde(alias = "")]
    None,

    Slow,
    Fast,
    Focus,
    Burst,
}


/// Release sets [Card]s may belong to.
#[non_exhaustive]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum CardSet {
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
}
