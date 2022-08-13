//! This module defines the [`main`] function for [`patched_porobot_telegram`].

use crate::data::corebundle::globals::LocalizedGlobalsIndexes;
use crate::data::corebundle::CoreBundle;
use crate::data::setbundle::card::{Card, CardIndex};
use crate::data::setbundle::SetBundle;
use crate::search::cardsearch::CardSearchEngine;
use crate::telegram::handler::{inline_query_handler, message_handler};
use log::*;
use std::path::PathBuf;
use teloxide::prelude::*;

/// The main function that [`patched_porobot_telegram`] should run when it's started.
pub async fn main() {
    pretty_env_logger::init();
    debug!("Logger initialized successfully!");

    debug!("Loading bundles...");
    let core = CoreBundle::load(&*PathBuf::from("./data/core-en_us"))
        .expect("to be able to load `core-en_us` bundle");
    let set1 = SetBundle::load(&*PathBuf::from("./data/set1-en_us"))
        .expect("to be able to load `set1-en_us` bundle");
    let set2 = SetBundle::load(&*PathBuf::from("./data/set2-en_us"))
        .expect("to be able to load `set2-en_us` bundle");
    let set3 = SetBundle::load(&*PathBuf::from("./data/set3-en_us"))
        .expect("to be able to load `set3-en_us` bundle");
    let set4 = SetBundle::load(&*PathBuf::from("./data/set4-en_us"))
        .expect("to be able to load `set4-en_us` bundle");
    let set5 = SetBundle::load(&*PathBuf::from("./data/set5-en_us"))
        .expect("to be able to load `set5-en_us` bundle");
    let set6 = SetBundle::load(&*PathBuf::from("./data/set6-en_us"))
        .expect("to be able to load `set6-en_us` bundle");
    debug!("Loaded all bundles!");

    debug!("Indexing globals...");
    let globals = LocalizedGlobalsIndexes::from(core.globals);
    debug!("Indexed globals!");

    debug!("Indexing cards...");
    let cards: Vec<Card> = [
        set1.cards, set2.cards, set3.cards, set4.cards, set5.cards, set6.cards,
    ]
    .concat();

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
