//! Module defining helper functions for returning [inline mode] results.
//!
//! [inline mode]: https://core.telegram.org/bots/api#inline-mode

use crate::data::corebundle::globals::LocalizedGlobalsIndexes;
use crate::data::setbundle::card::Card;
use crate::telegram::display::display_card;
use teloxide::types::{InlineQueryResult, InlineQueryResultPhoto, ParseMode};

/// Converts a [Card] into a [InlineQueryResult].
pub fn card_to_inlinequeryresult(
    globals: &LocalizedGlobalsIndexes,
    card: &Card,
) -> InlineQueryResult {
    InlineQueryResult::Photo(InlineQueryResultPhoto {
        id: card.code.to_owned(),
        title: Some(card.name.to_owned()),
        caption: Some(display_card(&globals, &card)),
        parse_mode: Some(ParseMode::Html),
        photo_url: card
            .main_art()
            .expect("Card to have at least one illustration")
            .card_jpg()
            .parse()
            .expect("Card to have a valid card_jpg URL"),
        thumb_url: card
            .main_art()
            .expect("Card to have at least one illustration")
            .card_jpg()
            .parse()
            .expect("Card to have a valid card_jpg URL"),
        photo_width: Some(680),
        photo_height: Some(1024),
        description: None,
        caption_entities: None,
        reply_markup: None,
        input_message_content: None,
    })
}
