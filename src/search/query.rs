use log::*;
use std::collections::HashMap;
use tantivy::Index;
use tantivy::IndexReader;
use tantivy::LeasedItem;
use tantivy::Searcher;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Schema;
use crate::data::schema::Card;
use itertools::Itertools;


pub fn build_query_parser(index: &Index) -> QueryParser {
    let schema = index.schema();

    let name = schema.get_field("name").unwrap();
    let description = schema.get_field("description").unwrap();
    let code = schema.get_field("code").unwrap();

    QueryParser::for_index(&index, vec![name, description, code])
}


pub fn build_searcher(reader: &IndexReader) -> LeasedItem<Searcher> {
    reader.searcher()
}


pub fn search_card(schema: &Schema, parser: &QueryParser, reader: &IndexReader, map: &HashMap<String, Card>, q: &str) -> Vec<Card> {
    debug!("Searching for `{}`...", &q);

    let code = schema.get_field("code").unwrap();

    debug!("Building Card searcher...");
    let searcher = build_searcher(reader);
    let query = parser.parse_query(q)
        .expect("to be able to parse the query");
    let search = searcher.search(&*query, &TopDocs::with_limit(50))
        .expect("to be able to search for a card");
    debug!("Retrieved {} results!", &search.len());

    search.iter().filter_map(|(_score, address)| searcher.doc(address.to_owned()).ok())
        .filter_map(|doc| doc.get_first(code).cloned())
        .filter_map(|field| field.as_text().map(String::from))
        .filter_map(|code| map.get(&*code))
        .cloned()
        .collect_vec()
}