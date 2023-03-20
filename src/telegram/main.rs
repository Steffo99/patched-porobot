//! Module defining the [`main`] function for `patched_porobot_telegram`.

use crate::data::corebundle::create_globalindexes_from_wd;
use crate::data::setbundle::create_cardindex_from_wd;
use crate::search::cardsearch::CardSearchEngine;
use crate::telegram::handler::{inline_query_handler, message_handler};
use log::*;
use rand::Rng;
use teloxide::prelude::*;

/// The function that `patched_porobot_telegram` should run when it's started.
pub async fn main() {
    pretty_env_logger::init();
    debug!("Logger initialized successfully!");

    debug!("Creating LocalizedGlobalIndexes...");
    let globals = create_globalindexes_from_wd();
    debug!("Created LocalizedGlobalIndexes!");

    debug!("Creating CardIndex...");
    let cards = create_cardindex_from_wd();
    debug!("Created CardIndex!");

    debug!("Creating CardSearchEngine...");
    let engine = CardSearchEngine::new(globals, cards);
    debug!("Created CardSearchEngine!");

    debug!("Creating Telegram bot with parameters from the environment...");
    let bot = Bot::from_env();
    let me = bot
        .get_me()
        .send()
        .await
        .expect("Telegram bot parameters to be valid");
    debug!("Created Telegram bot!");

    debug!("Generating crystal for this run...");
    let rng = rand::thread_rng();
    let crystal: String = String::from_utf8(rng.sample_iter(&rand::distributions::Alphanumeric).take(6).collect()).unwrap();
    debug!("Generated crystal: {}", &crystal);

    debug!("Creating handlers...");
    let handler = dptree::entry()
        .branch(inline_query_handler(crystal, engine))
        .branch(message_handler());
    debug!("Created handlers!");

    info!(
        "@{} is ready!",
        &me.username.as_ref().expect("bot to have an username")
    );
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
