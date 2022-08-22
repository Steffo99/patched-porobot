//! This module defines the [`main`] function for `patched_porobot_telegram`.

use crate::data::corebundle::globals::LocalizedGlobalsIndexes;
use crate::data::corebundle::CoreBundle;
use crate::data::setbundle::card::{Card, CardIndex};
use crate::data::setbundle::SetBundle;
use crate::search::cardsearch::CardSearchEngine;
use crate::telegram::handler::{inline_query_handler, message_handler};
use glob::glob;
use log::*;
use std::path::PathBuf;
use teloxide::prelude::*;

/// The main function that `patched_porobot_telegram` should run when it's started.
pub async fn main() {
    pretty_env_logger::init();
    debug!("Logger initialized successfully!");

    debug!("Loading core bundle...");
    let core = CoreBundle::load(&*PathBuf::from("./data/core-en_us"))
        .expect("to be able to load `core-en_us` bundle");
    debug!("Loaded core bundle successfully!");

    debug!("Loading set bundles...");
    let setpaths = glob("./data/set*-*")
        .expect("setglob to be a valid glob")
        .into_iter()
        .filter(|sp| sp.is_ok())
        .map(|sp| sp.unwrap());
    let mut cards: Vec<Card> = vec![];
    for setpath in setpaths {
        debug!("Loading {:?}...", &setpath);
        let set = SetBundle::load(&setpath)
            .expect(&*format!("to be able to load {:?} as a set bundle", &setpath));
        let mut setcards = set.cards;
        cards.append(&mut setcards);
    }
    debug!("Loaded {} cards!", &cards.len());

    debug!("Indexing globals...");
    let globals = LocalizedGlobalsIndexes::from(core.globals);
    debug!("Indexed globals!");

    let mut index = CardIndex::new();
    for card in cards {
        index.insert(card.code.clone(), card);
    }
    let cards = index;
    debug!("Indexed cards!");

    debug!("Creating search engine...");
    let engine = CardSearchEngine::new(globals, cards);
    debug!("Created search engine!");

    debug!("Creating Telegram bot with parameters from the environment...");
    let bot = Bot::from_env();
    let me = bot
        .get_me()
        .send()
        .await
        .expect("Telegram bot parameters to be valid");
    debug!("Created Telegram bot!");

    debug!("Creating handlers...");
    let handler = dptree::entry()
        .branch(inline_query_handler(engine))
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
