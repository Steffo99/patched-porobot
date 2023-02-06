use std::collections::HashMap;
use std::env;
use itertools::Itertools;
use serenity::builder::EditInteractionResponse;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::application::interaction::*;
use serenity::model::application::interaction::{InteractionResponseType, Interaction};
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use tantivy::query::QueryParserError;
use crate::data::setbundle::card::Card;
use crate::data::setbundle::keyword::CardKeyword;
use crate::data::setbundle::rarity::CardRarity;
use crate::data::setbundle::region::CardRegion;
use crate::data::setbundle::set::CardSet;
use crate::search::cardsearch::CardSearchEngine;

pub struct EventHandler;

impl EventHandler {
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

        let result = match engine.query(&query, 1) {
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
                    e
                        .title(card.name.clone())
                        .field("Set", match card.set {
                            CardSet::Foundations => "<:foundations:1071644734667366410> Foundations",
                            CardSet::RisingTides => "<:rising_tides:1071644736126976160> Rising Tides",
                            CardSet::CallOfTheMountain => "<:call_of_the_mountain:1071644738555478076> Call of the Mountain",
                            CardSet::EmpiresOfTheAscended => "<:empires_of_the_ascended:1071644740342255616> Empires of the Ascended",
                            CardSet::BeyondTheBandlewood => "<:beyond_the_bandlewood:1071644742640750734> Beyond the Bandlewood",
                            CardSet::Worldwalker => "<:worldwalker:1071644743798370315> Worldwalker",
                            CardSet::TheDarkinSaga => "<:the_darkin_saga:1071644746411417610> The Darkin Saga",
                            CardSet::Events => "Events", // TODO: Add icon
                            CardSet::Unsupported => "<:invaliddeck:1056022952396730438> Unknown",
                        }, true)
                        .field("Regions", card.regions.iter().map(|r| match r {
                            CardRegion::Noxus => "<:noxus:1056022924169064498> Noxus",
                            CardRegion::Demacia => "<:demacia:1056023014128484412> Demacia",
                            CardRegion::Freljord => "<:freljord:1056024331437735936> Freljord",
                            CardRegion::ShadowIsles => "<:shadowisles:1056022886848135292> Shadow Isles",
                            CardRegion::Targon => "<:targon:1056022866174418944> Targon",
                            CardRegion::Ionia => "<:ionia:1056022949569777708> Ionia",
                            CardRegion::Bilgewater => "<:bilgewater:1056024288215437484> Bilgewater",
                            CardRegion::Shurima => "<:shurima:1056022884616765500> Shurima",
                            CardRegion::PiltoverZaun => "<:piltoverzaun:1056022918959734835> Piltover & Zaun",
                            CardRegion::BandleCity => "<:bandlecity:1056024280493735976> Bandle City",
                            CardRegion::Runeterra => "<:runeterra:1056022895031238727> Runeterra",
                            CardRegion::Unsupported => "<:invaliddeck:1056022952396730438> Unknown",
                        }).join(", "), true)
                        .color(match card.rarity {
                            CardRarity::None => 0x000000,
                            CardRarity::Common => 0x1e6a49,
                            CardRarity::Rare => 0x244778,
                            CardRarity::Epic => 0x502970,
                            CardRarity::Champion => 0x81541f,
                        })
                        .field("Rarity", match card.rarity {
                            CardRarity::None => "None",
                            CardRarity::Common => "<:common:1056024315046412358> Common",
                            CardRarity::Rare => "<:rare:1056022907433799690> Rare",
                            CardRarity::Epic => "<:epic:1056023004028608622> Epic",
                            CardRarity::Champion => "<:champion:1056024303856001034> Champion",
                        }, true)
                        // TODO: .field("Types")
                        .field("Mana cost", format!("{} mana", card.cost), true)
                        .field("Stats", format!("{} | {}", card.attack, card.health), true)
                        // TODO
                        .field("Keywords", card.keywords.iter().map(|r| match r {
                            CardKeyword::SpellOverwhelm => "SpellOverwhelm",
                            CardKeyword::Burst => "Burst",
                            CardKeyword::Countdown => "Countdown",
                            CardKeyword::OnPlay => "OnPlay",
                            CardKeyword::Landmark => "Landmark",
                            CardKeyword::Shurima => "Shurima",
                            CardKeyword::Attach => "Attach",
                            CardKeyword::Noxus => "Noxus",
                            CardKeyword::Fleeting => "Fleeting",
                            CardKeyword::ClobberNoEmptySlotRequirement => "ClobberNoEmptySlotRequirement",
                            CardKeyword::Nab => "Nab",
                            CardKeyword::Focus => "Focus",
                            CardKeyword::Enlightened => "Enlightened",
                            CardKeyword::Invoke => "Invoke",
                            CardKeyword::Boon => "Boon",
                            CardKeyword::Trap => "Trap",
                            CardKeyword::Drain => "Drain",
                            CardKeyword::LastBreath => "LastBreath",
                            CardKeyword::Demacia => "Demacia",
                            CardKeyword::BandleCity => "BandleCity",
                            CardKeyword::Fast => "Fast",
                            CardKeyword::Bilgewater => "Bilgewater",
                            CardKeyword::Runeterra => "Runeterra",
                            CardKeyword::Recall => "Recall",
                            CardKeyword::Weakest => "Weakest",
                            CardKeyword::Support => "Support",
                            CardKeyword::Slow => "Slow",
                            CardKeyword::Obliterate => "Obliterate",
                            CardKeyword::Imbue => "Imbue",
                            CardKeyword::Targon => "Targon",
                            CardKeyword::ShadowIsles => "ShadowIsles",
                            CardKeyword::AuraVisualFakeKeyword => "AuraVisualFakeKeyword",
                            CardKeyword::Ionia => "Ionia",
                            CardKeyword::Nightfall => "Nightfall",
                            CardKeyword::PiltoverZaun => "PiltoverZaun",
                            CardKeyword::Attune => "Attune",
                            CardKeyword::Daybreak => "Daybreak",
                            CardKeyword::SilenceIndividualKeyword => "SilenceIndividualKeyword",
                            CardKeyword::Skill => "Skill",
                            CardKeyword::Plunder => "Plunder",
                            CardKeyword::DoubleAttack => "DoubleAttack",
                            CardKeyword::Vulnerable => "Vulnerable",
                            CardKeyword::Elusive => "Elusive",
                            CardKeyword::Stun => "Stun",
                            CardKeyword::Fated => "Fated",
                            CardKeyword::BlockElusive => "BlockElusive",
                            CardKeyword::Fury => "Fury",
                            CardKeyword::Barrier => "Barrier",
                            CardKeyword::Immobile => "Immobile",
                            CardKeyword::Hallowed => "Hallowed",
                            CardKeyword::Evolve => "Evolve",
                            CardKeyword::Frostbite => "Frostbite",
                            CardKeyword::Overwhelm => "Overwhelm",
                            CardKeyword::QuickAttack => "QuickAttack",
                            CardKeyword::Tough => "Tough",
                            CardKeyword::Regeneration => "Regeneration",
                            CardKeyword::Silenced => "Silenced",
                            CardKeyword::SpellShield => "SpellShield",
                            CardKeyword::Lifesteal => "Lifesteal",
                            CardKeyword::Augment => "Augment",
                            CardKeyword::Impact => "Impact",
                            CardKeyword::Scout => "Scout",
                            CardKeyword::Ephemeral => "Ephemeral",
                            CardKeyword::Lurk => "Lurk",
                            CardKeyword::Formidable => "Formidable",
                            CardKeyword::Challenger => "Challenger",
                            CardKeyword::Fearsome => "Fearsome",
                            CardKeyword::CantBlock => "CantBlock",
                            CardKeyword::Deep => "Deep",
                            CardKeyword::Flow => "Flow",
                            CardKeyword::Equipment => "Equipment",
                            CardKeyword::Unsupported => "<:invaliddeck:1056022952396730438> Unknown",
                        }).join(", "), false)
                        .field("Effect", card.localized_description_text.clone(), false)
                        .footer(|f| f.text(card.localized_flavor_text.clone()));

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

    pub fn command_deck<'r>(ctx: &Context, response: &'r mut EditInteractionResponse, options: HashMap<String, Option<CommandDataOptionValue>>) -> &'r mut EditInteractionResponse {
        response.content(":warning: `/deck` is not yet implemented")
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

                command.edit_original_interaction_response(
                    &ctx.http,
                    |response| match *cmd_name {
                        "card" => Self::command_card(&ctx, response, cmd_opts),
                        "deck" => Self::command_deck(&ctx, response, cmd_opts),
                        _ => response.content(":warning: Unknown command."),
                    }
                ).await.expect("to be able to update the deferred response");
            }
            _ => {}
        }
    }
}