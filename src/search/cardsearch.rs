//! Module defining a search engine to find [Card]s.

use crate::data::corebundle::globals::LocalizedGlobalsIndexes;
use crate::data::setbundle::card::{Card, CardIndex};
use itertools::Itertools;
use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, QueryParserError};
use tantivy::schema::{Field, NumericOptions, Schema, TextOptions};
use tantivy::tokenizer::TextAnalyzer;
use tantivy::{Document, Index, IndexReader, IndexWriter};
use crate::data::setbundle::code::CardCode;

/// The search engine.
///
/// To create a new engine, use [CardSearchEngine::new].
///
/// A separate search engine should be created for every locale.
pub struct CardSearchEngine {
    /// The index of the search engine.
    index: Index,

    /// Struct to read documents from the search engine.
    reader: IndexReader,

    /// Struct to parse queries input by the user.
    parser: QueryParser,

    /// Localization of game globals used by the search engine.
    pub globals: LocalizedGlobalsIndexes,

    /// Cards searchable in the search engine.
    pub cards: CardIndex,
}

impl CardSearchEngine {
    /// Create the [tantivy::tokenizer::TextAnalyzer] for card text.
    ///
    /// It should not alter text significantly, as it may contain important game vocabulary terms.
    fn tokenizer() -> TextAnalyzer {
        use tantivy::tokenizer::*;

        TextAnalyzer::from(SimpleTokenizer).filter(LowerCaser)
    }

    /// Create the [tantivy::schema::TextOptions] for card codes.
    ///
    /// Card codes should:
    /// - TODO: be tokenized without alterations;
    /// - ignore positioning;
    /// - be retrievable (what [tantivy] calls "stored").
    fn options_code() -> TextOptions {
        use tantivy::schema::*;

        TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("card")
                    .set_index_option(IndexRecordOption::Basic),
            )
            .set_stored()
            .set_fast()
    }

    /// Create the [tantivy::schema::TextOptions] for card keywords.
    ///
    /// Card keywords should:
    /// - be tokenized with the [CardSearchEngine::tokenizer];
    /// - ignore positioning.
    fn options_keyword() -> TextOptions {
        use tantivy::schema::*;

        TextOptions::default().set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("card")
                .set_index_option(IndexRecordOption::Basic),
        )
    }

    /// Create the [tantivy::schema::TextOptions] for card text fields.
    ///
    /// Card text should:
    /// - TODO: be tokenized with the tokenizer for the locale language;
    /// - consider both frequency and positioning.
    fn options_text() -> TextOptions {
        use tantivy::schema::*;

        TextOptions::default().set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("card")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        )
    }

    /// Create the [tantivy::schema::NumericOptions] for card numeric fields.
    ///
    /// Card numbers should:
    /// - be indexed.
    fn options_number() -> NumericOptions {
        use tantivy::schema::*;

        NumericOptions::default().set_indexed()
    }

    /// Create the [Schema] for the search engine.
    ///
    /// It will contain [Field]s with the following names:
    ///
    /// | Name          | Type                             | Description |
    /// |---------------|----------------------------------|-------------|
    /// | `code`        | [code](Self::options_code)       | The internal [card code](Card::code), such as `01IO012`. |
    /// | `name`        | [text](Self::options_text)       | The [name of the card](Card::name). |
    /// | `type`        | [keyword](Self::options_keyword) | The [type of the card](Card::type), such as `Unit`. |
    /// | `set`         | [keyword](Self::options_keyword) | The [set the card belongs to](Card::set), such as `Beyond the Bandlewood`. |
    /// | `rarity`      | [keyword](Self::options_keyword) | The [rarity of the card](Card::rarity), such as `Rare`, or `Champion`. |
    /// | `collectible` | [number](Self::options_number)   | `1` if the [card is collectible](Card::collectible), `0` otherwise. |
    /// | `regions`     | [keyword](Self::options_keyword) | The [regions of the card](Card::regions), separated by spaces. |
    /// | `attack`      | [number](Self::options_number)   | The [attack of the unit](Card::attack); always `0` for non-units. |
    /// | `cost`        | [number](Self::options_number)   | The [mana cost of the card](Card::cost); always `0` for non-playable cards. |
    /// | `health`      | [number](Self::options_number)   | The [health of the unit](Card::health); always `0` for non-units. |
    /// | `spellspeed`  | [keyword](Self::options_keyword) | The [speed of the spell](Card::spell_speed), or an empty string for non-spells. Note that [`Focus`](crate::data::setbundle::keyword::CardKeyword::Focus) is a keyword, and not a spell speed. |
    /// | `keywords`    | [keyword](Self::options_keyword) | The [keywords of the card](Card::keywords), separated by spaces. |
    /// | `description` | [text](Self::options_text)       | The [description of the card](Card::localized_description_text). |
    /// | `levelup`     | [text](Self::options_text)       | The [level up text of the champion](Card::localized_levelup_text). |
    /// | `flavor`      | [text](Self::options_text)       | The [flavor text of the card](Card::localized_flavor_text). |
    /// | `artist`      | [text](Self::options_text)       | The [artist(s) of the card's illustration](Card::artist_name). |
    ///
    /// Use [Self::schema_fields] to create the [CardSchemaFields] object containing all of them.
    ///
    fn schema() -> Schema {
        use tantivy::schema::*;

        let mut schema_builder = Schema::builder();

        let options_code = Self::options_code();
        let options_keyword = Self::options_keyword();
        let options_text = Self::options_text();
        let options_number = Self::options_number();

        schema_builder.add_text_field("code", options_code);
        schema_builder.add_text_field("name", options_text.clone());
        schema_builder.add_text_field("type", options_keyword.clone());
        schema_builder.add_text_field("set", options_keyword.clone());
        schema_builder.add_text_field("rarity", options_keyword.clone());
        schema_builder.add_u64_field("collectible", options_number.clone());
        schema_builder.add_text_field("regions", options_keyword.clone());
        schema_builder.add_u64_field("attack", options_number.clone());
        schema_builder.add_u64_field("cost", options_number.clone());
        schema_builder.add_u64_field("health", options_number);
        schema_builder.add_text_field("spellspeed", options_keyword.clone());
        schema_builder.add_text_field("keywords", options_keyword.clone());
        schema_builder.add_text_field("description", options_text.clone());
        schema_builder.add_text_field("levelup", options_text.clone());
        schema_builder.add_text_field("flavor", options_text.clone());
        schema_builder.add_text_field("artist", options_text);
        schema_builder.add_text_field("subtypes", options_keyword.clone());
        schema_builder.add_text_field("supertype", options_keyword);

        schema_builder.build()
    }

    /// Create a [CardSchemaFields] object from the given schema.
    fn schema_fields(schema: &Schema) -> CardSchemaFields {
        CardSchemaFields {
            code: schema
                .get_field("code")
                .expect("schema to have a 'code' field"),
            name: schema
                .get_field("name")
                .expect("schema to have a 'name' field"),
            r#type: schema
                .get_field("type")
                .expect("schema to have a 'type' field"),
            set: schema
                .get_field("set")
                .expect("schema to have a 'set' field"),
            rarity: schema
                .get_field("rarity")
                .expect("schema to have a 'rarity' field"),
            collectible: schema
                .get_field("collectible")
                .expect("schema to have a 'collectible' field"),
            regions: schema
                .get_field("regions")
                .expect("schema to have a 'regions' field"),
            attack: schema
                .get_field("attack")
                .expect("schema to have a 'attack' field"),
            cost: schema
                .get_field("cost")
                .expect("schema to have a 'cost' field"),
            health: schema
                .get_field("health")
                .expect("schema to have a 'health' field"),
            spellspeed: schema
                .get_field("spellspeed")
                .expect("schema to have a 'spellspeed' field"),
            keywords: schema
                .get_field("keywords")
                .expect("schema to have a 'keywords' field"),
            description: schema
                .get_field("description")
                .expect("schema to have a 'description' field"),
            levelup: schema
                .get_field("levelup")
                .expect("schema to have a 'levelup' field"),
            flavor: schema
                .get_field("flavor")
                .expect("schema to have a 'flavor' field"),
            artist: schema
                .get_field("artist")
                .expect("schema to have a 'artist' field"),
            subtypes: schema
                .get_field("subtypes")
                .expect("schema to have a 'subtypes' field"),
            supertype: schema
                .get_field("supertype")
                .expect("schema to have a 'supertype' field"),
        }
    }

    /// Build [in RAM](Index::create_in_ram) the [Index] of the search engine.
    fn index() -> Index {
        Index::create_in_ram(Self::schema())
    }

    /// Build a [IndexWriter] with the optimal configuration for the search engine.
    ///
    /// Uses 12 MB of RAM; do not lower below 3 MB, or it will panic!
    fn writer(index: &Index) -> IndexWriter {
        index
            .writer(12_000_000)
            .expect("to be able to create a IndexWriter")
    }

    /// Build a [IndexReader] with the optimal configuration for the search engine.
    fn reader(index: &Index) -> IndexReader {
        index
            .reader_builder()
            .reload_policy(tantivy::ReloadPolicy::Manual)
            .try_into()
            .expect("to be able to create a IndexReader")
    }

    /// Create a [Document] from a [Card].
    fn document(
        fields: &CardSchemaFields,
        globals: &LocalizedGlobalsIndexes,
        card: Card,
    ) -> Document {
        use tantivy::doc;

        doc!(
            fields.code => card.code.full,
            fields.name => card.name,
            fields.r#type => String::from(&card.r#type),
            fields.set => card.set
                .localized(&globals.sets)
                .map(|cs| cs.name.to_owned())
                .unwrap_or_else(String::new),
            fields.rarity => card.rarity
                .localized(&globals.rarities)
                .map(|cr| cr.name.to_owned())
                .unwrap_or_else(String::new),
            fields.collectible => if card.collectible {1u64} else {0u64},
            fields.regions => card.regions.iter()
                .map(|region| region
                    .localized(&globals.regions)
                    .map(|cr| cr.name.to_owned())
                    .unwrap_or_else(String::new)
                ).join(" "),
            fields.attack => card.attack,
            fields.cost => card.cost,
            fields.health => card.health,
            fields.spellspeed => card.spell_speed
                .localized(&globals.spell_speeds)
                .map(|ss| ss.name.to_owned())
                .unwrap_or_else(String::new),
            fields.keywords => card.keywords.iter()
                .map(|keyword| keyword
                    .localized(&globals.keywords)
                    .map(|ck| ck.name.to_owned())
                    .unwrap_or_else(String::new))
                .join(" "),
            fields.description => card.localized_description_text,
            fields.levelup => card.localized_levelup_text,
            fields.flavor => card.localized_flavor_text,
            fields.artist => card.artist_name,
            fields.subtypes => card.subtypes.join(" "),
            fields.supertype => card.supertype,
        )
    }

    /// Build the [QueryParser] of the search engine.
    fn parser(index: &Index, fields: CardSchemaFields) -> QueryParser {
        let mut parser = QueryParser::for_index(
            &index,
            vec![
                fields.code,
                fields.name,
                fields.description,
                fields.set,
                fields.regions,
                fields.flavor,
                fields.artist,
                fields.subtypes,
                fields.supertype,
            ],
        );
        parser.set_conjunction_by_default();
        parser.set_field_boost(fields.code, 3.0);
        parser.set_field_boost(fields.name, 3.0);
        parser.set_field_boost(fields.flavor, 0.7);
        parser
    }

    /// Create a new [CardSearchEngine].
    pub fn new(globals: LocalizedGlobalsIndexes, cards: CardIndex) -> Self {
        let index = Self::index();
        let schema = index.schema();
        let fields = Self::schema_fields(&schema);

        index.tokenizers().register("card", Self::tokenizer());

        let mut writer = Self::writer(&index);
        for card in cards.values() {
            let document = Self::document(&fields, &globals, card.clone());
            writer
                .add_document(document)
                .expect("IndexWriter threads to not panic or die before adding a document");
        }
        writer
            .commit()
            .expect("IndexWriter threads to not panic or die before commit");

        let parser = Self::parser(&index, fields);
        let reader = Self::reader(&index);

        Self {
            index,
            reader,
            parser,
            globals,
            cards,
        }
    }

    /// Perform a query on the search engine.
    pub fn query(&self, input: &str, top: usize) -> Result<Vec<&Card>, QueryParserError> {
        let searcher = self.reader.searcher();

        let query = self.parser.parse_query(input)?;

        let search = searcher
            .search(&*query, &TopDocs::with_limit(top))
            .expect("Searcher::search to never fail");

        let f_code = self
            .index
            .schema()
            .get_field("code")
            .expect("schema to have a 'code' field");

        let results = search
            .iter()
            .filter_map(|(_score, address)| searcher.doc(address.to_owned()).ok())
            .filter_map(|doc| doc.get_first(f_code).cloned())
            .filter_map(|field| field.as_text().map(String::from))
            .filter_map(|code| self.cards.get(&CardCode::from(code)))
            .collect_vec();

        Ok(results)
    }
}

/// Struct containing all retrieved [CardSearchEngine] [Field]s.
///
/// This makes it easier to pass them around without having to re-fetch them every time they are used.
#[derive(Clone, Debug)]
struct CardSchemaFields {
    /// [Card::code].
    pub code: Field,
    /// [Card::name].
    pub name: Field,
    /// English [Card::type].
    pub r#type: Field,
    /// Localized [Card::set].
    pub set: Field,
    /// Localized [Card::rarity].
    pub rarity: Field,
    /// `0` if the card is not [Card::collectible], `1` otherwise.
    pub collectible: Field,
    /// Space-separated localized [Card::regions].
    pub regions: Field,
    /// [Card::attack].
    pub attack: Field,
    /// [Card::cost].
    pub cost: Field,
    /// [Card::health].
    pub health: Field,
    /// [Card::spell_speed].
    pub spellspeed: Field,
    /// Space-separated localized [Card::keywords].
    pub keywords: Field,
    /// [Card::localized_description_text].
    pub description: Field,
    /// [Card::localized_levelup_text].
    pub levelup: Field,
    /// [Card::localized_flavor_text].
    pub flavor: Field,
    /// [Card::artist_name].
    pub artist: Field,
    /// Space-separated [Card::subtypes].
    pub subtypes: Field,
    /// [Card::supertype].
    pub supertype: Field,
}
