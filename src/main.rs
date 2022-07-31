use teloxide::prelude::*;
use teloxide::types::*;
use log::*;

mod data;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("patched-porobot is starting...");

    let bot = Bot::from_env().auto_send();

    let handler = Update::filter_inline_query().branch(dptree::endpoint(|query: InlineQuery, bot: AutoSend<Bot>| async move {
        let result = InlineQueryResult::Article(InlineQueryResultArticle::new("test", "Test", InputMessageContent::Text(InputMessageContentText::new("Qui è dove metterei la mia carta, se solo ne avessi una!"))));

        let response = bot.answer_inline_query(&query.id, vec![result]).await;
        respond(())
    }));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}
