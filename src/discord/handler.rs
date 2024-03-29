//! Module containing the [`EventHandler`] and its associated functions.

use std::collections::{HashMap, HashSet};
use std::env;
use itertools::Itertools;
use serenity::builder::EditInteractionResponse;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::application::interaction::{InteractionResponseType, Interaction};
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use crate::data::deckcode::deck::Deck;
use crate::data::deckcode::format::DeckCodeFormat;
use crate::data::setbundle::r#type::CardType;
use crate::data::setbundle::rarity::CardRarity;
use crate::data::setbundle::supertype::CardSupertype;
use crate::search::cardsearch::CardSearchEngine;

/// Event handler for the bot.
///
/// Contains the functions that process events received by Discord.
pub struct EventHandler;

const WELCOME_MESSAGE: &str = r#"
👋 Hi! I'm a robotic poro who can search for Legends of Runeterra cards to send them in chats!

To search for a card, enter `/card` in a channel where I am enabled, then specify **your search query** as the `query` parameter, like this:
```text
/card query:mighty poro
```
After a while, I'll send in the channel the best match I can find for your query!

You can also perform more **complex queries**, such as this one:
```text
/card query:cost:4 AND attack:7 AND health:7
```
To read all details on the queries you can ask me to perform, visit the documentation at: <https://docs.rs/patched_porobot/latest/patched_porobot_discord>

Additionally, you can send me the `/deck` command together with a deck code to send the full deck details in chat, like this:
```
/deck code:CECQCAQCA4AQIAYKAIAQGLRWAQAQECAPEUXAIAQDAEBQOCIBAIAQEMJYAA
```
Have a fun time playing Legends of Runeterra!

_Patched Porobot isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties. Riot Games, and all associated properties are trademarks or registered trademarks of Riot Games, Inc._
"#;

impl EventHandler {
    /// Handle the `/help` command.
    pub fn command_help(response: &mut EditInteractionResponse) -> &mut EditInteractionResponse {
        response.content(WELCOME_MESSAGE);
        response
    }

    /// Handle the `/card` command.
    pub fn command_card<'r>(ctx: &Context, response: &'r mut EditInteractionResponse, options: HashMap<String, Option<CommandDataOptionValue>>) -> &'r mut EditInteractionResponse {
        let typemap = ctx.data.try_read().expect("to be able to acquire read lock on CardSearchEngine");
        let engine = typemap.get::<CardSearchEngine>().expect("CardSearchEngine to be in the TypeMap");

        let query = match options.get("query") {
            Some(q) => q,
            None => return response.content(":warning: Missing `query` parameter."),
        };

        let query = match query {
            Some(q) => q,
            None => return response.content(":warning: Empty `query` parameter."),
        };

        let query = match query {
            CommandDataOptionValue::String(q) => q,
            _ => return response.content(":warning: Invalid `query` parameter type."),
        };

        let result = match engine.query(query, 1) {
            Ok(r) => r,
            Err(_) => return response.content(":warning: Invalid card search query syntax."),
        };

        let result = result.get(0);

        match result {
            Some(card) => {
                let response = match card.main_art() {
                    Some(art) => response.content(art.card_png.clone()),
                    None => response.content(card.name.clone()),
                };
                let response = response.embed(|e| {
                    e.title(card.name.clone());

                    if !card.localized_description_text.is_empty() {
                        e.description(card.localized_description_text.clone());
                    }

                    if !card.keywords.is_empty() {
                        e.field("Keywords", card.keywords.iter().map(|r| {
                            let icon = r.discord_emoji();
                            let text = r.localized(&engine.globals.keywords).map_or_else(|| String::from("Unknown"), |l| l.name.clone());
                            format!("{icon} {text}")
                        }).join(", "), true);
                    }

                    e.field("Mana cost", format!("{} mana", card.cost), true);

                    if card.r#type == CardType::Unit {
                        e.field("Stats", format!("{} | {}", card.attack, card.health), true);
                    }

                    e.field("Types", {
                        let mut vec = Vec::new();
                        if card.supertype != CardSupertype::None {
                            vec.push(String::from(&card.supertype));
                        }
                        vec.push(String::from(&card.r#type));
                        for subtype in card.subtypes.iter() {
                            vec.push(subtype.to_owned())
                        };
                        vec
                    }.join(", "), true);

                    e.field("Regions", card.regions.iter().map(|r| {
                        let icon = r.discord_emoji();
                        let text = r.localized(&engine.globals.regions).map_or_else(|| String::from("Unknown"), |r| r.name.clone());
                        format!("{icon} {text}")
                    }).join(", "), false);

                    e.field("Set", {
                        let icon = card.set.discord_emoji();
                        let text = card.set.localized(&engine.globals.sets).map_or_else(|| String::from("Unknown"), |r| r.name.clone());
                        format!("{icon} {text}")
                    }, true);

                    let actual_rarity = match card.supertype {
                        CardSupertype::Champion => CardRarity::Champion,
                        _ => card.rarity
                    };

                    e.field("Rarity", {
                        let icon = actual_rarity.discord_emoji();
                        let text = actual_rarity.localized(&engine.globals.rarities).map_or_else(|| String::from("Unknown"), |r| r.name.clone());
                        format!("{icon} {text}")
                    } , true);

                    e.color(actual_rarity.color());

                    if !card.localized_flavor_text.is_empty() {
                        e.footer(|f| f.text(card.localized_flavor_text.clone()));
                    }

                    if let Some(art) = card.main_art() {
                        e
                            .field("Illustration by", card.artist_name.clone(), false)
                            .image(art.full_png.clone());
                    };

                    e
                });
                response
            }
            None => {
                response.content(":warning: No cards found.")
            }
        }
    }

    /// Handle the `/deck` command.
    pub fn command_deck<'r>(ctx: &Context, response: &'r mut EditInteractionResponse, options: HashMap<String, Option<CommandDataOptionValue>>) -> &'r mut EditInteractionResponse {
        let typemap = ctx.data.try_read().expect("to be able to acquire read lock on CardSearchEngine");
        let engine = typemap.get::<CardSearchEngine>().expect("CardSearchEngine to be in the TypeMap");

        let code = match options.get("code") {
            Some(c) => c,
            None => return response.content(":warning: Missing `code` parameter."),
        };

        let code = match code {
            Some(c) => c,
            None => return response.content(":warning: Empty `code` parameter."),
        };

        let code = match code {
            CommandDataOptionValue::String(c) => c,
            _ => return response.content(":warning: Invalid `code` parameter type."),
        };

        let deck = match Deck::from_code(code) {
            Ok(deck) => deck,
            _ => return response.content(":warning: Invalid deck code."),
        };

        let name = match options.get("name") {
            Some(Some(CommandDataOptionValue::String(n))) => Some(n),
            _ => None,
        };

        response.content(
            match name {
                Some(name) => format!("__**{}**__\n```text\n{}\n```", name, deck.to_code(DeckCodeFormat::F1).expect("to be able to serialize the deck code")),
                None => format!("```text\n{}\n```", deck.to_code(DeckCodeFormat::F1).expect("to be able to serialize the deck code")),
            });

        let (format, regions) = if let Some(regions) = deck.standard(&engine.cards) {
            ("<:standard:1095374776492638208> Standard 4.3", regions)
        } else if let Some(regions) = deck.eternal(&engine.cards) {
            ("<:eternal:1095374779130839151> Eternal", regions)
        } else if let Some(regions) = deck.unlimited_champions(&engine.cards) {
            ("Unlimited Champions", regions)
        } else if let Some(regions) = deck.singleton(&engine.cards) {
            ("Singleton", regions)
        } else {
            ("Unknown", HashSet::new())
        };

        response.embed(|e| {
            e.description(
                deck.contents.iter()
                    .map(|(cc, qty)| {
                        (cc.to_card(&engine.cards), qty)
                    })
                    .map(|(c, qty)| {
                        let name = match c {
                            None => String::from("<:invaliddeck:1056022952396730438> Unknown card"),
                            Some(c) => c.name.clone(),
                        };
                        format!("**{}×** {}", qty, name)
                    })
                    .join("\n")
            );

            e.field("Format", format, true);

            if !regions.is_empty() {
                e.field("Regions",
                    regions
                        .iter()
                        .map(|region| format!(
                            "{} {}",
                            region.discord_emoji(),
                            region.localized(&engine.globals.regions)
                                .map_or_else(|| String::from("Missing translation"), |l| l.name.clone())
                        ))
                        .join(", "),
                    false);
            }

            e
        })
    }

    /// Register the Slash Commands supported by this bot.
    ///
    /// If `SERENITY_DEV_GUILD_ID` is set, register them as guild commands to avoid caching, otherwise, register them as global commands.
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
                    )
                ).await?;
                guild.create_application_command(&ctx.http, |c| c
                    .name("deck")
                    .description("Send a deck in the chat.")
                    .create_option(|o| o
                        .kind(command::CommandOptionType::String)
                        .name("code")
                        .description("The code of the deck to send.")
                        .required(true)
                    )
                    .create_option(|o| o
                        .kind(command::CommandOptionType::String)
                        .name("name")
                        .description("The name of the deck.")
                        .required(false)
                    )
                ).await?;
                guild.create_application_command(&ctx.http, |c| c
                    .name("help")
                    .description("View the help message.")
                ).await?;
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
                    )
                ).await?;
                command::Command::create_global_application_command(&ctx.http, |c| c
                    .name("deck")
                    .description("Send a deck in the chat.")
                    .create_option(|o| o
                        .kind(command::CommandOptionType::String)
                        .name("code")
                        .description("The code of the deck to send.")
                        .required(true)
                    )
                    .create_option(|o| o
                        .kind(command::CommandOptionType::String)
                        .name("name")
                        .description("The name of the deck.")
                        .required(false)
                    )
                ).await?;
                command::Command::create_global_application_command(&ctx.http, |c| c
                    .name("help")
                    .description("View the help message.")
                ).await?;
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

    #[allow(clippy::single_match)]
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                let cmd_name = command.data.name.as_str();
                let cmd_opts: HashMap<String, Option<CommandDataOptionValue>> = command.data.options
                    .clone()
                    .into_iter()
                    .map(|option| (option.name, option.resolved))
                    .collect();

                log::info!("Received command: {}", &cmd_name);

                command.create_interaction_response(&ctx.http, |r| r
                    .interaction_response_data(|d| d
                        .ephemeral(cmd_name == "help")
                    )
                    .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                ).await.expect("to be able to defer the response");

                command.edit_original_interaction_response(
                    &ctx.http,
                    |response| match cmd_name {
                        "card" => Self::command_card(&ctx, response, cmd_opts),
                        "deck" => Self::command_deck(&ctx, response, cmd_opts),
                        "help" => Self::command_help(response),
                        _ => response.content(":warning: Unknown command."),
                    }
                ).await.expect("to be able to update the deferred response");
            }
            _ => {}
        }
    }
}