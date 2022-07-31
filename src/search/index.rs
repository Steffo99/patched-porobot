use tantivy::*;
use crate::data::schema::Card;
use crate::search::schema;


/// Build a [tantivy] [Index] storing [Card]s as documents.
pub fn build_card_index() -> Index {
    Index::create_in_ram(schema::build_card_schema())
}


/// Build a [tantivy] [IndexWriter] from the given [Index] with some preset parameters.
///
/// Currently allocates 4 MB.
pub fn build_writer(index: &Index) -> IndexWriter {
    index.writer(4_000_000)
        .expect("to be able to allocate a tantivy writer")
}


/// Write a [Vec] of [Card]s to a [tantivy] [Index], using a writer built with [build_writer].
pub fn write_cards_to_index(index: &Index, cards: &Vec<Card>) -> () {
    let schema = index.schema();
    let mut writer = build_writer(index);

    for card in cards {
        let document = schema::card_to_document(&schema, card.to_owned());
        writer.add_document(document)
            .expect("to be able to add a document to the index");
    }

    writer.commit()
        .expect("to be able to commit the schema changes");
}


/// Build a [tantivy] [IndexReader] from the given [Index] with some preset parameters.
pub fn build_reader(index: &Index) -> IndexReader {
    index.reader_builder().reload_policy(ReloadPolicy::OnCommit).try_into()
        .expect("to be able to allocate a tantivy reader")
}
