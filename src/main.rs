#[macro_use] extern crate tantivy;
#[macro_use] extern crate log;

use teloxide::prelude::*;
use teloxide::types::*;
use log::*;
use tantivy::{IndexReader, ReloadPolicy};

mod data;
mod search;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    debug!("Loading Set Bundles...");
    let cards = data::load::load_setbundles_infallible("./data/*/en_us/data/*-en_us.json");

    debug!("Creating Tantivy index...");
    let card_index = search::index::build_card_index();

    debug!("Writing cards to the index...");
    search::index::write_cards_to_index(&card_index, &cards);

    debug!("Creating Tantivy reader...");
    let card_reader = search::index::build_reader(&card_index);

    let inline_query_closure = |query: InlineQuery, bot: AutoSend<Bot>| async move {
        bot.answer_inline_query(&query.id, handle_query(&query, &card_reader)).await;
        respond(())
    };

    info!("patched-porobot is starting...");

    let bot = Bot::from_env().auto_send();

    let handler = Update::filter_inline_query().branch(dptree::endpoint(inline_query_closure));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}


/// Handle a [InlineQuery] incoming from Telegram.
fn handle_query(query: &InlineQuery, reader: &IndexReader) -> Vec<InlineQueryResult> {
    debug!("Creating Tantivy searcher...");
    let card_searcher = reader.searcher();

    let result = InlineQueryResult::Article(InlineQueryResultArticle::new("test", "Test", InputMessageContent::Text(InputMessageContentText::new("Qui è dove metterei la mia carta, se solo ne avessi una!"))));

    vec![result]
}