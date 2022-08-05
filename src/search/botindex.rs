//! This module provides functions to manage [tantivy] [Index]es for internal bot usage.

use tantivy::{Index, IndexReader, IndexWriter, LeasedItem, ReloadPolicy, Searcher};
use crate::search::card::card_schema;


/// Build a new [Index] for [crate::schena::setbundle::Card] documents, based on [card_schema].
pub fn card_index() -> Index {
    Index::create_in_ram(
        card_schema()
    )
}


/// Build a [IndexWriter] with the optimal configuration for [crate::schena::setbundle::Card] documents.
pub fn card_writer(index: &Index) -> IndexWriter {
    index
        .writer(4_000_000)
        .expect("to be able to allocate 4 MB for a IndexWriter")
}


/// Build a [IndexReader] with the optimal configuration for [crate::schena::setbundle::Card] documents.
pub fn card_reader(index: &Index) -> IndexReader {
    index
        .reader_builder()
        .reload_policy(ReloadPolicy::Manual)
        .try_into()
        .expect("to be able to create a IndexReader")
}
