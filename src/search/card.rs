//! This module configures [tantivy] structs for [Card] search.

use crate::load::corebundle::MappedGlobals;
use crate::schema::setbundle::{Card, CardType};


/// Create a new [tantivy::tokenizer::TextAnalyzer] for card text.
///
/// It should not alter text significantly, as it may contain important game vocabulary terms.
pub fn card_tokenizer() -> tantivy::tokenizer::TextAnalyzer {
    use tantivy::tokenizer::*;

    TextAnalyzer::from(SimpleTokenizer)
        .filter(LowerCaser)
}


/// Create a new [tantivy::schema::TextOptions] for card codes, skipping tokenization.
pub fn cardcode_options() -> tantivy::schema::TextOptions {
    use tantivy::schema::*;

    TextOptions::default()
        .set_stored()
        .set_fast()
}


/// Create a new [tantivy::schema::TextOptions] for card keywords, using the given tokenizer.
pub fn cardkeyword_options(tokenizer_name: &'static str) -> tantivy::schema::TextOptions {
    use tantivy::schema::*;

    TextOptions::default()
        .set_indexing_options(TextFieldIndexing::default()
            .set_tokenizer(tokenizer_name)
            .set_fieldnorms(false)
            .set_index_option(IndexRecordOption::Basic)
        )
}


/// Create a new [tantivy::schema::TextOptions] for card text fields, using the given tokenizer.
pub fn cardtext_options(tokenizer_name: &'static str) -> tantivy::schema::TextOptions {
    use tantivy::schema::*;

    TextOptions::default()
        .set_indexing_options(TextFieldIndexing::default()
            .set_tokenizer(tokenizer_name)
            .set_fieldnorms(true)
            .set_index_option(IndexRecordOption::WithFreqsAndPositions)
        )
}


/// Create a new [tantivy::schema::Schema] using [Card]s as documents.
pub fn card_schema(tokenizer_name: &'static str) -> tantivy::schema::Schema {
    use tantivy::schema::*;

    let mut schema_builder = Schema::builder();

    let cardcode: TextOptions = cardcode_options();
    let cardkeyword: TextOptions = cardkeyword_options(tokenizer_name);
    let cardtext: TextOptions = cardtext_options(tokenizer_name);

    schema_builder.add_text_field("code", cardcode);
    schema_builder.add_text_field("name", cardtext.clone());
    schema_builder.add_text_field("type", cardkeyword.clone());
    schema_builder.add_text_field("set", cardkeyword.clone());
    schema_builder.add_text_field("rarity", cardkeyword.clone());
    schema_builder.add_u64_field("collectible", INDEXED);
    schema_builder.add_text_field("regions", cardkeyword.clone());
    schema_builder.add_u64_field("attack", INDEXED);
    schema_builder.add_u64_field("cost", INDEXED);
    schema_builder.add_u64_field("health", INDEXED);
    schema_builder.add_text_field("spellspeed", cardkeyword.clone());
    schema_builder.add_text_field("keywords", cardkeyword.clone());
    schema_builder.add_text_field("description", cardtext.clone());
    schema_builder.add_text_field("levelup", cardtext.clone());
    schema_builder.add_text_field("associated", cardtext.clone());
    schema_builder.add_text_field("flavor", cardtext.clone());
    schema_builder.add_text_field("artist", cardtext);
    schema_builder.add_text_field("subtypes", cardkeyword.clone());
    schema_builder.add_text_field("supertype", cardkeyword);

    schema_builder.build()
}


/// Create a new [tantivy::Document] using a [Card] in a specific [locale](MappedGlobals] as base.
pub fn card_to_document(schema: &tantivy::schema::Schema, locale: &MappedGlobals, card: Card) -> tantivy::Document {
    use tantivy::*;
    use itertools::Itertools;

    let f_code = schema.get_field("code").expect("schema to have a 'code' field");
    let f_name = schema.get_field("name").expect("schema to have a 'name' field");
    let f_type = schema.get_field("type").expect("schema to have a 'type' field");
    let f_set = schema.get_field("set").expect("schema to have a 'set' field");
    let f_rarity = schema.get_field("rarity").expect("schema to have a 'rarity' field");
    let f_collectible = schema.get_field("collectible").expect("schema to have a 'collectible' field");
    let f_regions = schema.get_field("regions").expect("schema to have a 'regions' field");
    let f_attack = schema.get_field("attack").expect("schema to have a 'attack' field");
    let f_cost = schema.get_field("cost").expect("schema to have a 'cost' field");
    let f_health = schema.get_field("health").expect("schema to have a 'health' field");
    let f_spellspeed = schema.get_field("spellspeed").expect("schema to have a 'spellspeed' field");
    let f_keywords = schema.get_field("keywords").expect("schema to have a 'keywords' field");
    let f_description = schema.get_field("description").expect("schema to have a 'description' field");
    let f_levelup = schema.get_field("levelup").expect("schema to have a 'levelup' field");
    let f_associated = schema.get_field("associated").expect("schema to have a 'associated' field");
    let f_flavor = schema.get_field("flavor").expect("schema to have a 'flavor' field");
    let f_artist = schema.get_field("artist").expect("schema to have a 'artist' field");
    let f_subtypes = schema.get_field("subtypes").expect("schema to have a 'subtypes' field");
    let f_supertype = schema.get_field("supertype").expect("schema to have a 'supertype' field");

    let c_type = match card.r#type {
        CardType::Spell => "Spell",
        CardType::Unit => "Unit",
        CardType::Ability => "Ability",
        CardType::Landmark => "Landmark",
        CardType::Trap => "Trap",
        CardType::Unsupported => ""
    };

    doc!(
        f_code => card.code,
        f_name => card.name,
        f_type => c_type,
        f_set => card.set
            .localized(&locale.sets)
            .map(|cs| cs.name.to_owned())
            .unwrap_or_else(String::new),
        f_rarity => card.rarity
            .localized(&locale.rarities)
            .map(|cr| cr.name.to_owned())
            .unwrap_or_else(String::new),
        f_collectible => if card.collectible {1u64} else {0u64},
        f_regions => card.regions.iter()
            .map(|region| region
                .localized(&locale.regions)
                .map(|cr| cr.name.to_owned())
                .unwrap_or_else(String::new)
            ).join(" "),
        f_attack => card.attack,
        f_cost => card.cost,
        f_health => card.health,
        f_spellspeed => card.spell_speed
            .localized(&locale.spell_speeds)
            .map(|ss| ss.name.to_owned())
            .unwrap_or_else(String::new),
        f_keywords => card.keywords.iter()
            .map(|keyword| keyword
                .localized(&locale.keywords)
                .map(|ck| ck.name.to_owned())
                .unwrap_or_else(String::new))
            .join(" "),
        f_description => card.localized_description_text,
        f_levelup => card.localized_levelup_text,
        f_associated => card.associated_card_codes.join(" "),
        f_flavor => card.localized_flavor_text,
        f_artist => card.artist_name,
        f_subtypes => card.subtypes.join(" "),
        f_supertype => card.supertype,
    )
}


/// Stage all [tantivy::Document]s generated from [Card]s contained in the passed [Vec] for write on a [tantivy::Index] via the given [tantivy::IndexWriter].
pub fn cards_to_index(writer: tantivy::IndexWriter, schema: tantivy::schema::Schema, locale: MappedGlobals, cards: Vec<Card>) -> tantivy::Result<()> {
    for card in cards {
        writer.add_document(card_to_document(&schema, &locale, card))?;
    };
    Ok(())
}
