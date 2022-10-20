//! Module providing handlers for @patchedporobot on Telegram.

use crate::data::deckcode::deck::Deck;
use crate::search::cardsearch::CardSearchEngine;
use crate::telegram::inline::{card_to_inlinequeryresult, deck_to_inlinequeryresult};
use itertools::Itertools;
use log::*;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::payloads::{AnswerInlineQuery, SendMessage};
use teloxide::prelude::*;
use teloxide::requests::{JsonRequest, ResponseResult};
use teloxide::types::{ParseMode, Recipient};
use lazy_static::lazy_static;
use regex::Regex;

/// Handle inline queries by searching cards on the [CardSearchEngine].
pub fn inline_query_handler(
    engine: CardSearchEngine,
) -> Handler<'static, DependencyMap, ResponseResult<()>, DpHandlerDescription> {
    Update::filter_inline_query().chain(dptree::endpoint(move |query: InlineQuery, bot: Bot| {
        info!("Handling inline query...");

        // It's not a real loop, it's just to make the code flow more tolerable.
        let payload: AnswerInlineQuery = loop {
            if query.query.len() == 0 {
                debug!("Empty query specified.");
                break AnswerInlineQuery {
                    inline_query_id: query.id.clone(),
                    results: vec![],
                    cache_time: None,
                    is_personal: Some(false),
                    next_offset: None,
                    switch_pm_text: Some("How to use the bot".to_string()),
                    switch_pm_parameter: Some("err-no-query".to_string()),
                };
            }

            lazy_static! {
                static ref DECK_RE: Regex = Regex::new(r#"^(?P<code>[ABCDEFGHIJKLMNOPQRSTUVWXYZ234567]+)(?:\s+(?P<name>.+?))?\s*$"#).unwrap();
            }
            
            if let Some(deck_captures) = DECK_RE.captures(&query.query) {
                if let Some(deck_code) = deck_captures.name("code") {
                    if let Ok(deck) = Deck::from_code(&deck_code.as_str()) {
                        
                        debug!("Parsed deck successfully!");
                        let name = deck_captures.name("name").map(|m| m.as_str());

                        break AnswerInlineQuery {
                            inline_query_id: query.id.clone(),
                            results: vec![deck_to_inlinequeryresult(&engine.cards, &deck, &name)],
                            cache_time: None,
                            is_personal: Some(false),
                            next_offset: None,
                            switch_pm_text: None,
                            switch_pm_parameter: None,
                        };
                    }
                }
            }

            debug!("Querying the card search engine...");
            let results = engine.query(&query.query, 50);

            if let Err(_) = results {
                debug!("Invalid card search query syntax.");
                break AnswerInlineQuery {
                    inline_query_id: query.id.clone(),
                    results: vec![],
                    cache_time: None,
                    is_personal: Some(false),
                    next_offset: None,
                    switch_pm_text: Some("Invalid query syntax".to_string()),
                    switch_pm_parameter: Some("err-invalid-query".to_string()),
                };
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
                };
            }

            debug!("Card search query successful!");
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
            };
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

const WELCOME_MESSAGE: &'static str = r#"
👋 Hi! I'm a robotic poro who can search for Legends of Runeterra cards to send them in chats!

To search for a card, enter <b>my username</b> in any chat, followed by <b>your search query</b>, like this:
<pre>@patchedporobot braum</pre>

After a while, you'll see the list of cards I found, and you'll be able to tap one of them to send it!

You can also perform more <b>complex queries</b>, such as this one:
<pre>@patchedporobot cost:4 AND attack:7 AND health:7</pre>

To read all details on the queries you can ask me to perform, visit the <a href="https://docs.rs/patched_porobot/latest/patched_porobot_telegram/">documentation</a>!

Additionally, you can ask me to read a deck code to send the full deck details in chat, like this:
<pre>@patchedporobot CECQCAQCA4AQIAYKAIAQGLRWAQAQECAPEUXAIAQDAEBQOCIBAIAQEMJYAA</pre>

Have a fun time playing Legends of Runeterra!

<i>@patchedporobot isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties. Riot Games, and all associated properties are trademarks or registered trademarks of Riot Games, Inc.</i>
"#;

/// Handle all messages by replying with the help text.
pub fn message_handler() -> Handler<'static, DependencyMap, ResponseResult<()>, DpHandlerDescription>
{
    Update::filter_message().chain(dptree::endpoint(move |message: Message, bot: Bot| {
        info!("Handling private message: `{:?}`", &message.text());

        let payload = SendMessage {
            chat_id: Recipient::Id(message.chat.id.clone()),
            text: WELCOME_MESSAGE.to_string(),
            parse_mode: Some(ParseMode::Html),
            entities: None,
            disable_web_page_preview: Some(true),
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
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
