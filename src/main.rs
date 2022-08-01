#[macro_use] extern crate tantivy;

use teloxide::prelude::*;
use log::*;

mod data;
mod search;
mod telegram;


/// Run the bot.
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    debug!("Loading Set Bundles...");
    let cards = data::load::load_setbundles_infallible("./data/*/en_us/data/set*-en_us.json");
    debug!("Loaded {} cards!", &cards.len());

    debug!("Creating Card index...");
    let card_index = search::index::build_card_index();
    debug!("Card index created successfully!");

    debug!("Creating Card hashmap...");
    let card_map = data::map::build_card_code_hashmap(&cards);
    debug!("Card hashmap contains {} cards!", &card_map.len());

    debug!("Writing Cards to the Card index...");
    search::index::write_cards_to_index(&card_index, &cards);
    debug!("Wrote Cards to the Card index!");

    debug!("Creating Card query parser...");
    let card_query_parser = search::query::build_query_parser(&card_index);
    debug!("Created Card query parser!");

    debug!("Creating Card reader...");
    let card_reader = search::index::build_reader(&card_index);
    debug!("Created Card reader!");

    debug!("Creating Telegram Bot with parameters from the environment...");
    let bot = Bot::from_env().auto_send();
    let bot_me = bot.get_me().await.expect("Telegram Bot parameters to be valid");
    debug!("Created Telegram Bot @{}!", &bot_me.username.as_ref().unwrap());

    let handler = Update::filter_inline_query().branch(dptree::endpoint(move |query: InlineQuery, bot: AutoSend<Bot>| {
        let card_schema = card_index.schema();
        let card_query_parser = &card_query_parser;
        let card_reader = &card_reader;
        let card_map = &card_map;

        let results = search::query::search_card(
            &card_schema,
            &card_query_parser,
            &card_reader,
            &card_map,
            &query.query
        );

        async move {
            let reply = results.iter()
                .map(telegram::result::create_inlinequeryresult);

            if let Err(e) = bot.answer_inline_query(&query.id, reply).send().await {
                error!("{:?}", e);
            };
            respond(())
        }
    }));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}
