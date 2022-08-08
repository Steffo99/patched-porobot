#[cfg(not(feature = "telegram"))]
fn main() {
    println!("The `telegram` feature was not included on compilation, therefore this binary is not available.")
}

#[cfg(feature = "telegram")]
#[tokio::main]
async fn main() {
    use std::path::PathBuf;
    use log::*;
    use patched_porobot::data::setbundle::card::{Card, CardIndex};
    use patched_porobot::data::corebundle::CoreBundle;
    use patched_porobot::data::setbundle::SetBundle;
    use patched_porobot::data::corebundle::globals::LocalizedGlobalsIndexes;
    use patched_porobot::search::cardsearch::CardSearchEngine;
    use patched_porobot::telegram::inline::card_to_inlinequeryresult;
    use teloxide::payloads::{AnswerInlineQuery, SendMessage};
    use teloxide::requests::JsonRequest;
    use teloxide::types::{Recipient, ParseMode};
    use teloxide::prelude::*;
    use itertools::Itertools;

    pretty_env_logger::init();
    debug!("Logger initialized successfully!");

    debug!("Loading bundles...");
    let core = CoreBundle::load(&*PathBuf::from("./card-data/core-en_us")).expect("to be able to load `core-en_us` bundle");
    let set1 =  SetBundle::load(&*PathBuf::from("./card-data/set1-en_us")).expect("to be able to load `set1-en_us` bundle");
    let set2 =  SetBundle::load(&*PathBuf::from("./card-data/set2-en_us")).expect("to be able to load `set2-en_us` bundle");
    let set3 =  SetBundle::load(&*PathBuf::from("./card-data/set3-en_us")).expect("to be able to load `set3-en_us` bundle");
    let set4 =  SetBundle::load(&*PathBuf::from("./card-data/set4-en_us")).expect("to be able to load `set4-en_us` bundle");
    let set5 =  SetBundle::load(&*PathBuf::from("./card-data/set5-en_us")).expect("to be able to load `set5-en_us` bundle");
    let set6 =  SetBundle::load(&*PathBuf::from("./card-data/set6-en_us")).expect("to be able to load `set6-en_us` bundle");
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

    debug!("Creating message handler...");
    let message_handler = Update::filter_message().chain(dptree::endpoint(move |message: Message, bot: Bot| {
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
    }));
    debug!("Created message handler!");

    debug!("Creating inline query handler...");
    let inline_query_handler = Update::filter_inline_query().chain(dptree::endpoint(move |query: InlineQuery, bot: Bot| {
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
    }));
    debug!("Create inline query handler!");

    debug!("Merging handlers...");
    let handler = dptree::entry()
        .branch(inline_query_handler)
        .branch(message_handler);
    debug!("Merged handlers!");

    info!("@{} is ready!", &me.username.as_ref().expect("bot to have an username"));
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
