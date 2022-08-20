//! Module defining helper functions for returning [inline mode] results.
//!
//! [inline mode]: https://core.telegram.org/bots/api#inline-mode

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::ptr::hash;
use crate::data::corebundle::globals::LocalizedGlobalsIndexes;
use crate::data::setbundle::card::{Card, CardIndex};
use crate::telegram::display::{display_card, display_deck};
use teloxide::types::{InlineQueryResult, InlineQueryResultArticle, InlineQueryResultPhoto, InputMessageContent, InputMessageContentText, ParseMode};
use crate::data::deckcode::deck::Deck;
use crate::data::deckcode::format::DeckCodeFormat;

/// Converts a [Card] into a [InlineQueryResult].
pub fn card_to_inlinequeryresult(
    globals: &LocalizedGlobalsIndexes,
    card: &Card,
) -> InlineQueryResult {
    InlineQueryResult::Photo(InlineQueryResultPhoto {
        id: card.code.full.to_owned(),
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


pub fn deck_to_inlinequeryresult(index: &CardIndex, deck: &Deck) -> InlineQueryResult {
    let code = deck.to_code(DeckCodeFormat::F1).expect("serialized deck to deserialize properly");

    InlineQueryResult::Article(InlineQueryResultArticle {
        id: format!("{:x}", md5::compute(&code)),
        title: format!("Deck with {} cards", deck.contents.len()),
        input_message_content: InputMessageContent::Text(InputMessageContentText {
            message_text: display_deck(index, deck, code),
            parse_mode: Some(ParseMode::Html),
            entities: None,
            disable_web_page_preview: Some(true)
        }),
        reply_markup: None,
        url: None,
        hide_url: None,
        description: None,
        thumb_url: None,
        thumb_width: None,
        thumb_height: None
    })
}