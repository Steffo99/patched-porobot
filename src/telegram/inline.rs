//! Module defining helper functions for returning [inline mode] results.
//!
//! [inline mode]: https://core.telegram.org/bots/api#inline-mode

use crate::data::corebundle::globals::LocalizedGlobalsIndexes;
use crate::data::deckcode::deck::Deck;
use crate::data::deckcode::format::DeckCodeFormat;
use crate::data::setbundle::card::{Card, CardIndex};
use crate::telegram::display::{display_card, display_deck};
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InlineQueryResultPhoto, InputMessageContent,
    InputMessageContentText, ParseMode,
};

/// Convert a [Card] into a [InlineQueryResult].
pub fn card_to_inlinequeryresult(
    crystal: &str,
    globals: &LocalizedGlobalsIndexes,
    card: &Card,
) -> InlineQueryResult {
    InlineQueryResult::Photo(InlineQueryResultPhoto {
        id: format!("{}:{}", &crystal, &card.code.full),
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

/// Convert a [Deck] with an optional name into a [InlineQueryResult].
pub fn deck_to_inlinequeryresult(
    crystal: &str,
    index: &CardIndex,
    deck: &Deck,
    name: &Option<&str>
) -> InlineQueryResult {
    let code = deck
        .to_code(DeckCodeFormat::F1)
        .expect("serialized deck to deserialize properly");

    InlineQueryResult::Article(InlineQueryResultArticle {
        id: format!("{}:{:x}", &crystal, md5::compute(&code)),
        title: match &name {
            Some(name) => format!(r#"Deck "{}" with {} cards"#, name, deck.contents.len()),
            None => format!("Deck with {} cards", deck.contents.len())
        },
        input_message_content: InputMessageContent::Text(InputMessageContentText {
            message_text: display_deck(index, deck, &code, &name),
            parse_mode: Some(ParseMode::Html),
            entities: None,
            disable_web_page_preview: Some(true),
        }),
        reply_markup: None,
        url: None,
        hide_url: None,
        description: None,
        thumb_url: None,
        thumb_width: None,
        thumb_height: None,
    })
}
