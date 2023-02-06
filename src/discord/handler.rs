use std::collections::HashMap;
use std::env;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::application::interaction::*;
use serenity::model::application::interaction::{InteractionResponseType, Interaction};
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub struct EventHandler;

impl EventHandler {
    pub async fn command_card(ctx: &Context, command: ApplicationCommandInteraction, options: HashMap<String, Option<CommandDataOptionValue>>) -> anyhow::Result<()> {
        command.edit_original_interaction_response(
            &ctx.http,
            |r| r.content(":warning: `/card` is not implemented yet.")
        ).await.expect("to be able to update the deferred response");

        Ok(())
    }

    pub async fn command_deck(ctx: &Context, command: ApplicationCommandInteraction, options: HashMap<String, Option<CommandDataOptionValue>>) -> anyhow::Result<()> {
        command.edit_original_interaction_response(
            &ctx.http,
            |r| r.content(":warning: `/card` is not implemented yet.")
        ).await.expect("to be able to update the deferred response");

        Ok(())
    }

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
                let response = command.create_interaction_response(&ctx.http, |r| r
                    .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                ).await.expect("to be able to defer the response");

                let cmd_name = &command.data.name.as_str();
                let cmd_opts: HashMap<String, Option<CommandDataOptionValue>> = command.data.options
                    .clone()
                    .into_iter()
                    .map(|option| (option.name, option.resolved))
                    .collect();

                log::info!("Received command: {}", &cmd_name);


                let result: anyhow::Result<()> = match *cmd_name {
                    "card" => Self::command_card(&ctx, command, cmd_opts).await,
                    "deck" => Self::command_deck(&ctx, command, cmd_opts).await,

                    _ => {
                        command.edit_original_interaction_response(
                            &ctx.http,
                            |r| r.content(":warning: Unknown command.")
                        ).await.expect("to be able to update the deferred response");
                        Ok(())
                    }
                };

                if let Err(error) = result {
                    log::error!("{}", error.to_string())
                }
            }
            _ => {}
        }
    }
}