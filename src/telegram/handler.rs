//! Module providing handlers for @patchedporobot on Telegram.

use itertools::Itertools;
use log::*;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::payloads::{AnswerInlineQuery, SendMessage};
use teloxide::requests::{JsonRequest, ResponseResult};
use teloxide::prelude::*;
use teloxide::types::{ParseMode, Recipient};
use crate::search::cardsearch::CardSearchEngine;
use crate::telegram::inline::card_to_inlinequeryresult;


/// Handle inline queries by searching cards on the [CardSearchEngine].
pub fn inline_query_handler(engine: CardSearchEngine) -> Handler<'static, DependencyMap, ResponseResult<()>, DpHandlerDescription> {
    Update::filter_inline_query().chain(dptree::endpoint(move |query: InlineQuery, bot: Bot| {
        info!("Handling inline query: `{}`", &query.query);

        // It's not a real loop, it's just to make the code flow more tolerable.
        let payload: AnswerInlineQuery = loop {
            if query.query.len() == 0 {
                break AnswerInlineQuery {
                    inline_query_id: query.id.clone(),
                    results: vec![],
                    cache_time: None,
                    is_personal: Some(false),
                    next_offset: None,
                    switch_pm_text: Some("Open help message".to_string()),
                    switch_pm_parameter: Some("err-no-query".to_string()),
                }
            }

            debug!("Querying the search engine...");
            let results = engine.query(&query.query, 50);

            if let Err(_) = results {
                debug!("Invalid query syntax.");
                break AnswerInlineQuery {
                    inline_query_id: query.id.clone(),
                    results: vec![],
                    cache_time: None,
                    is_personal: Some(false),
                    next_offset: None,
                    switch_pm_text: Some("Invalid query syntax".to_string()),
                    switch_pm_parameter: Some("err-invalid-query".to_string()),
                }
            }
            let results = results.unwrap();

            let len = results.len();
            if len == 0 {
                debug!("No cards found.");
                break AnswerInlineQuery {
                    inline_query_id: query.id.clone(),
                    results: vec![],
                    cache_time: None,
                    is_personal: Some(false),
                    next_offset: None,
                    switch_pm_text: Some("No cards found".to_string()),
                    switch_pm_parameter: Some("err-no-results".to_string()),
                }
            }

            debug!("Found {} cards.", &len);
            break AnswerInlineQuery {
                inline_query_id: query.id.clone(),
                results: results
                    .iter()
                    .map(|card| card_to_inlinequeryresult(&engine.globals, card))
                    .collect_vec(),
                cache_time: Some(300),
                is_personal: Some(false),
                next_offset: None,
                switch_pm_text: None,
                switch_pm_parameter: None,
            }
        };

        async move {
            let telegram_reply = JsonRequest::new(bot.clone(), payload).send().await;

            if let Err(e) = telegram_reply {
                error!("{:?}", &e);
            }

            respond(())
        }
    }))
}


/// Handle all messages by replying with the help text.
pub fn message_handler() -> Handler<'static, DependencyMap, ResponseResult<()>, DpHandlerDescription> {
    Update::filter_message().chain(dptree::endpoint(move |message: Message, bot: Bot| {
        info!("Handling private message: `{:?}`", &message.text());

        let payload = SendMessage {
            chat_id: Recipient::Id(message.chat.id.clone()),
            // TODO: Add a proper message here.
            text: "TODO: Introduction message!".to_string(),
            parse_mode: Some(ParseMode::Html),
            entities: None,
            disable_web_page_preview: Some(true),
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None
        };

        async move {
            let telegram_reply = JsonRequest::new(bot.clone(), payload).send().await;

            if let Err(e) = telegram_reply {
                error!("{:?}", &e);
            }

            respond(())
        }
    }))
}