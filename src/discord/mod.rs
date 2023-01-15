//! Module providing utilities to be used in the `patched_porobot_discord` executable target.
//!
//! While adding new features to this module, remember that binaries [can only access the public API of the crate](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries), as they considered a separate crate from the rest of the project.

use std::env;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::application::interaction::*;

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


pub struct EventHandler;

impl EventHandler {
    pub async fn register_commands(ctx: &Context) -> anyhow::Result<()> {
        match env::var("SERENITY_DEV_GUILD_ID") {
            Ok(guild) => {
                let guild: u64 = guild.parse().expect("SERENITY_DEV_GUILD_ID to be valid");
                let guild: GuildId = guild.into();

                guild.create_application_command(&ctx.http, |c| c
                    .name("card")
                    .description("Search and send a card in the chat.")
                    .create_option(|o| o
                        .kind(command::CommandOptionType::String)
                        .name("query")
                        .description("The query to send to the card search engine.")
                        .required(true)
                    )).await?;
                guild.create_application_command(&ctx.http, |c| c
                    .name("deck")
                    .description("Send a deck in the chat.")
                    .create_option(|o| o
                        .kind(command::CommandOptionType::String)
                        .name("code")
                        .description("The code of the deck to send.")
                        .required(true)
                    )).await?;
            }
            Err(_) => {
                command::Command::create_global_application_command(&ctx.http, |c| c
                    .name("card")
                    .description("Search and send a card in the chat.")
                    .create_option(|o| o
                        .kind(command::CommandOptionType::String)
                        .name("query")
                        .description("The query to send to the card search engine.")
                        .required(true)
                    )).await?;
                command::Command::create_global_application_command(&ctx.http, |c| c
                    .name("deck")
                    .description("Send a deck in the chat.")
                    .create_option(|o| o
                        .kind(command::CommandOptionType::String)
                        .name("code")
                        .description("The code of the deck to send.")
                        .required(true)
                    )).await?;
            }
        };

        Ok(())
    }
}

#[serenity::async_trait]
impl serenity::client::EventHandler for EventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::debug!("Received ready event from the gateway");

        EventHandler::register_commands(&ctx).await.expect("to be able to register commands");

        log::info!("{} is ready!", &ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                command.create_interaction_response(&ctx.http, |r| r
                    .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                ).await.expect("to be able to defer the response");

                let cmd_name = &command.data.name.as_str();
                let cmd_guild = &command.data.guild_id.expect("command to be sent in a guild");
                let cmd_opts = options_hashmap(command.data.options.clone());

                log::info!("Received command: {}", &cmd_name);

                let result = match *cmd_name {
                    "card" => todo!(),
                    "deck" => todo!(),
                    _ => {}
                };

                todo!();

                let content = match result {
                    Ok(t) => t,
                    Err(AngyError::User(t)) => format!(":warning: {}", &t),
                    Err(e) => {
                        log::error!("{e:?}");
                        format!(":no_entry: Unexpected error occurred:\n```rust\n{e:#?}\n```")
                    }
                };

                command.edit_original_interaction_response(&ctx.http, |r| r
                    .content(content)
                ).await.expect("to be able to update the deferred response");
            }
            _ => {}
        }
    }
}