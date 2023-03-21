//! Module defining the [`main`] function for `patched_porobot_discord`.

use std::env;
use log::*;
use serenity::prelude::*;
use crate::data::corebundle::create_globalindexes_from_dd_latest_en_us;
use crate::data::setbundle::create_cardindex_from_dd_latest_en_us;
use crate::discord::handler::EventHandler;
use crate::search::cardsearch::CardSearchEngine;

/// The function that `patched_porobot_discord` should run when it's started.
pub async fn main() {
    pretty_env_logger::init();
    debug!("Logger initialized successfully!");

    debug!("Creating LocalizedGlobalIndexes...");
    let globals = create_globalindexes_from_dd_latest_en_us().await;
    debug!("Created LocalizedGlobalIndexes!");

    debug!("Creating CardIndex...");
    let cards = create_cardindex_from_dd_latest_en_us().await;
    debug!("Created CardIndex!");

    debug!("Creating CardSearchEngine...");
    let engine = CardSearchEngine::new(globals, cards);
    debug!("Created CardSearchEngine!");

    let token: String = env::var("SERENITY_TOKEN").expect("SERENITY_TOKEN to be set");
    let appid: u64 = env::var("SERENITY_APPID").expect("SERENITY_APPID to be set")
        .parse().expect("SERENITY_APPID to be valid");

    Client::builder(&token, GatewayIntents::non_privileged())
        .event_handler(EventHandler)
        .type_map_insert::<CardSearchEngine>(engine)
        .application_id(appid)
        .await
        .expect("to be able to create the Discord client")
        .start_autosharded()
        .await
        .expect("to be able to start the Discord client");
}
