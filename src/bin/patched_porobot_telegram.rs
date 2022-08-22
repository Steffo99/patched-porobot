//! # [@patchedporobot]
//!
//! Inline bot for searching and sending Legends of Runeterra cards and decks in Telegram chats
//!
//! ## Usage
//!
//! [@patchedporobot] is a inline bot: this means that you can use it everywhere, without having to add it to chats.
//!
//! ### Card queries
//!
//! You can search for a card by entering in the "Write a message..." bot the username of the bot, followed by the card you want to search for:
//!
//! ```text
//! @patchedporobot braum
//! ```
//!
//! You can specify multiple words to find cards that contain all of them:
//!
//! ```text
//! @patchedporobot mighty poro
//! ```
//!
//! Terms will be searched in multiple fields, so to find [Daring Poro](https://leagueoflegends.fandom.com/wiki/Daring_Poro_(Legends_of_Runeterra)) you may search for:
//!
//! ```text
//! @patchedporobot piltover poro
//! ```
//!
//! #### Conjunctions
//!
//! By default, all terms in the query are joined by `AND` conjuctions, meaning that only cards containing **all** of the terms are retrieved.
//!
//! If you want to find cards matching **any** of the terms, you'll have to manually join them with the `OR` conjuction:
//!
//! ```text
//! @patchedporobot progress OR heimerdinger
//! ```
//!
//! To have both `AND`s and `OR`s in the same query you'll need to specify all of them explicitly:
//!
//! ```text
//! @patchedporobot von AND yipp OR cat
//! ```
//!
//! #### Fields
//!
//! You can perform searches about specific card properties:
//!
//! ```text
//! @patchedporobot cost:4 attack:7 health:7
//! ```
//!
//! Conjunctions are supported even when searching by specific fields:
//!
//! ```text
//! @patchedporobot name:Bard OR description:Chime
//! ```
//!
//! ##### Supported fields
//!
//! [@patchedporobot] supports the various fields for searching cards: see [**this table**](patched_porobot::search::cardsearch::CardSearchEngine::schema) for a list of all of them!
//!
//! #### Ranges
//!
//! Finally, you can request specific ranges for your search using square brackets and the `TO` keyword:
//!
//! ```text
//! @patchedporobot attack:[8 TO 12]
//! ```
//!
//! #### Query parser
//!
//! Since [@patchedporobot] uses [`tantivy`] internally, you might find more information on even more advanced queries in the [documentation of their `QueryParser`](tantivy::query::QueryParser)!
//!
//! ### Deck queries
//!
//! You can have [@patchedporobot] display a deck and its cards by pasting the deck code after the bot's username:
//!
//! ```text
//! @patchedporobot CIBQCAICAQAQGBQIBEBAMBAJBMGBUHJNGE4AEAIBAIYQEAQGEU2QCAIBAIUQ
//! ```
//!
//! Then, select the "Deck with N cards" option to send the deck's card list in the chat!
//!
//!
//! [@patchedporobot]: https://t.me/patchedporobot

#![doc(html_logo_url = "https://raw.githubusercontent.com/Steffo99/patched-porobot/main/icon.png")]

#[doc(hidden)]
#[tokio::main]
async fn main() {
    patched_porobot::telegram::main::main().await;
}
