//! # [@patchedporobot]
//!
//! Inline bot for searching and sending Legends of Runeterra cards in Telegram chats
//!
//! ## Usage
//!
//! [@patchedporobot] is a inline bot: this means that you can use it everywhere, without having to add it to chats.
//!
//! ### Basic queries
//!
//! You can search for a card by entering in the "Write a message..." bot the username of the bot, followed by the card you want to search for:
//!
//! ```text
//! @patchedporobot braum
//! ```
//!
//! You can specify multiple words to find cards that contain all of them:
//!
//! ```text
//! @patchedporobot mighty poro
//! ```
//!
//! Terms will be searched in multiple fields, so to find [Daring Poro](https://leagueoflegends.fandom.com/wiki/Daring_Poro_(Legends_of_Runeterra)) you may search for:
//!
//! ```text
//! @patchedporobot piltover poro
//! ```
//!
//! ### Conjunctions
//!
//! By default, all terms in the query are joined by `AND` conjuctions, meaning that only cards containing **all** of the terms are retrieved.
//!
//! If you want to find cards matching **any** of the terms, you'll have to manually join them with the `OR` conjuction:
//!
//! ```text
//! @patchedporobot progress OR heimerdinger
//! ```
//!
//! To have both `AND`s and `OR`s in the same query you'll need to specify all of them explicitly:
//!
//! ```text
//! @patchedporobot von AND yipp OR cat
//! ```
//!
//! ### Fields
//!
//! You can perform searches about specific card properties:
//!
//! ```text
//! @patchedporobot cost:4 attack:7 health:7
//! ```
//!
//! Conjunctions are supported even when searching by specific fields:
//!
//! ```text
//! @patchedporobot name:Bard OR description:Chime
//! ```
//!
//! #### Supported fields
//!
//! [@patchedporobot] supports the various fields for searching cards: see [**this table**](patched_porobot::search::cardsearch::CardSearchEngine::schema) for a list of all of them!
//!
//! ### Ranges
//!
//! Finally, you can request specific ranges for your search using square brackets and the `TO` keyword:
//!
//! ```text
//! @patchedporobot attack:[8 TO 12]
//! ```
//!
//! ### Query parser
//!
//! Since [@patchedporobot] uses [`tantivy`] internally, you might find more information on even more advanced queries in the [documentation of their `QueryParser`](tantivy::query::QueryParser)!
//!
//!
//! [@patchedporobot]: https://t.me/patchedporobot

#![doc(html_logo_url = "https://raw.githubusercontent.com/Steffo99/patched-porobot/main/icon.png")]

use std::path::PathBuf;
use log::*;
use patched_porobot::data::setbundle::card::{Card, CardIndex};
use patched_porobot::data::corebundle::CoreBundle;
use patched_porobot::data::setbundle::SetBundle;
use patched_porobot::data::corebundle::globals::LocalizedGlobalsIndexes;
use patched_porobot::search::cardsearch::CardSearchEngine;
use patched_porobot::telegram::handler::{inline_query_handler, message_handler};
use teloxide::prelude::*;


#[doc(hidden)]
#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    debug!("Logger initialized successfully!");

    debug!("Loading bundles...");
    let core = CoreBundle::load(&*PathBuf::from("./data/core-en_us")).expect("to be able to load `core-en_us` bundle");
    let set1 =  SetBundle::load(&*PathBuf::from("./data/set1-en_us")).expect("to be able to load `set1-en_us` bundle");
    let set2 =  SetBundle::load(&*PathBuf::from("./data/set2-en_us")).expect("to be able to load `set2-en_us` bundle");
    let set3 =  SetBundle::load(&*PathBuf::from("./data/set3-en_us")).expect("to be able to load `set3-en_us` bundle");
    let set4 =  SetBundle::load(&*PathBuf::from("./data/set4-en_us")).expect("to be able to load `set4-en_us` bundle");
    let set5 =  SetBundle::load(&*PathBuf::from("./data/set5-en_us")).expect("to be able to load `set5-en_us` bundle");
    let set6 =  SetBundle::load(&*PathBuf::from("./data/set6-en_us")).expect("to be able to load `set6-en_us` bundle");
    debug!("Loaded all bundles!");

    debug!("Indexing globals...");
    let globals = LocalizedGlobalsIndexes::from(core.globals);
    debug!("Indexed globals!");

    debug!("Indexing cards...");
    let cards: Vec<Card> = [
        set1.cards,
        set2.cards,
        set3.cards,
        set4.cards,
        set5.cards,
        set6.cards
    ].concat();

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
    let me = bot.get_me().send().await.expect("Telegram bot parameters to be valid");
    debug!("Created Telegram bot!");

    debug!("Creating handlers...");
    let handler = dptree::entry()
        .branch(inline_query_handler(engine))
        .branch(message_handler());
    debug!("Created handlers!");

    info!("@{} is ready!", &me.username.as_ref().expect("bot to have an username"));
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
