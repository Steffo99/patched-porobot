//! This module provides functions to perform queries for internal bot usage.


use tantivy::{Index, IndexReader, TantivyError};
use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, QueryParserError};
use tantivy::schema::Schema;
use itertools::Itertools;
use crate::search::botsearch::QueryError::Parsing;


pub fn card_query_parser(index: &Index) -> QueryParser {
    let schema = index.schema();

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

    QueryParser::for_index(
        &index,
        vec![
            f_code,
            f_name,
            f_type,
            f_set,
            f_rarity,
            f_collectible,
            f_regions,
            f_attack,
            f_cost,
            f_health,
            f_spellspeed,
            f_keywords,
            f_description,
            f_levelup,
            f_associated,
            f_flavor,
            f_artist,
            f_subtypes,
            f_supertype,
        ]
    )
}


pub enum QueryError {
    Parsing(QueryParserError),
    Search(TantivyError),
}


pub fn card_query(schema: &Schema, reader: &IndexReader, parser: &QueryParser, query: &str, amount: usize) -> Result<Vec<String>, QueryError> {
    log::debug!("Searching for `{}`...", &query);

    let searcher = reader.searcher();
    let query = parser.parse_query(query)
        .map_err(QueryError::Parsing)?;
    let search = searcher.search(&*query, &TopDocs::with_limit(amount))
        .map_err(QueryError::Search)?;

    let f_code = schema.get_field("code").expect("schema to have a 'code' field");

    let results = search.iter()
        .filter_map(|(_score, address)| searcher.doc(address.to_owned()).ok())
        .filter_map(|doc| doc.get_first(f_code).cloned())
        .filter_map(|field| field.as_text().map(String::from))
        .collect_vec();

    Ok(results)
}
