use tantivy::schema::*;
use crate::data::schema::Card;


/// Build a [tantivy] [Schema] storing [Card]s as documents.
///
/// TODO: Allow search on all fields.
pub fn build_card_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("name", TEXT);
    schema_builder.add_text_field("description", TEXT);
    schema_builder.add_text_field("code", STRING | STORED);
    schema_builder.build()
}


/// Convert a [Card] to a Tantivy [Document], using the specified [Schema] (which should come from [build_card_schema]).
pub fn card_to_document(schema: &Schema, card: Card) -> Document {
    let name = schema.get_field("name").unwrap();
    let description = schema.get_field("description").unwrap();
    let code = schema.get_field("code").unwrap();

    doc!(
        name => card.name,
        description => card.description_raw,
        code => card.card_code,
    )
}