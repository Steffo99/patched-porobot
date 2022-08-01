use crate::data::schema::Card;
use teloxide::types::*;
use reqwest::Url;
use crate::telegram::format::format_card;

pub fn create_inlinequeryresult(card: &Card) -> InlineQueryResult {
    InlineQueryResult::Photo(InlineQueryResultPhoto {
        id: card.card_code.to_owned(),
        title: Some(card.name.to_owned()),
        description: Some(card.description_raw.to_owned()),
        caption: Some(format_card(&card)),
        photo_url: Url::parse(
            &card.assets.get(0).expect("card to have assets").game_absolute_path).expect("card to have a valid asset URL"
        ),
        thumb_url: Url::parse(
            &card.assets.get(0).expect("card to have assets").game_absolute_path).expect("card to have a valid asset URL"
        ),
        photo_width: Some(680),
        photo_height: Some(1024),
        parse_mode: Some(ParseMode::Html),
        caption_entities: None,
        reply_markup: None,
        input_message_content: None,
    })
}
