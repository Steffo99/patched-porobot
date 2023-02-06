use std::env;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::application::interaction::*;
use crate::discord::handler::EventHandler;


pub async fn main() {
    let token: String = env::var("SERENITY_TOKEN").expect("SERENITY_TOKEN to be set");
    let appid: u64 = env::var("SERENITY_APPID").expect("SERENITY_APPID to be set")
        .parse().expect("SERENITY_APPID to be valid");

    Client::builder(&token, GatewayIntents::non_privileged())
        .event_handler(EventHandler)
        .application_id(appid)
        .await
        .expect("to be able to create the Discord client")
        .start_autosharded()
        .await
        .expect("to be able to start the Discord client");
}
