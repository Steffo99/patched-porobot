//! # [Patched Porobot#7556]
//!
//! Bot for searching and sending Legends of Runeterra cards and decks in Discord channels
//!
//! ## Usage
//!
//! [Patched Porobot#7556] is based on slash commands; you can add it to your server by clicking on the following link:
//!
//! * [Add to Server](https://discord.com/api/oauth2/authorize?client_id=1071989978743193672&scope=applications.commands)
//!
//! After adding it to your server, you can use its commands in channels by entering `/` in the message box and then selecting the command you want to send to the bot.
//!
//! ### Card queries
//!
//! You can search for a card by specifying the `/card` command and inserting the card's name as the `query` parameter:
//!
//! ```text
//! /card query:shadowshift
//! ```
//!
//! You can specify multiple words to find the card that contains all of them:
//!
//! ```text
//! /card query:mighty poro
//! ```
//!
//! Terms will be searched in multiple fields, so to find [Daring Poro](https://leagueoflegends.fandom.com/wiki/Daring_Poro_(Legends_of_Runeterra)) you may search for:
//!
//! ```text
//! /card query:piltover poro
//! ```
//!
//! Since different Champion cards have the same name, you may disambiguate them by entering `level:N` as part of the query:
//!
//! ```
//! /card query:braum level:2
//! ```
//!
//! #### Conjunctions
//!
//! By default, all terms in the query are joined by `AND` conjuctions, meaning that only cards containing **all** of the terms are retrieved.
//!
//! If you want to find cards matching **any** of the terms, you'll have to manually join them with the `OR` conjuction:
//!
//! ```text
//! /card query:progress OR heimerdinger
//! ```
//!
//! To have both `AND`s and `OR`s in the same query you'll need to specify all of them explicitly:
//!
//! ```text
//! /card query:von AND yipp OR cat
//! ```
//!
//! #### Fields
//!
//! You can perform searches about specific card properties:
//!
//! ```text
//! /card query:cost:4 attack:7 health:7
//! ```
//!
//! Conjunctions are supported even when searching by specific fields:
//!
//! ```text
//! /card query:name:Bard OR description:Chime
//! ```
//!
//! ##### Supported fields
//!
//! [Patched Porobot#7556] supports the various fields for searching cards: see [**this table**](patched_porobot::search::cardsearch::CardSearchEngine::schema) for a list of all of them!
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
//! Since [Patched Porobot#7556] uses [`tantivy`] internally, you might find more information on even more advanced queries in the [documentation of their `QueryParser`](tantivy::query::QueryParser)!
//!
//! ### Deck parsing
//!
//! You can have [Patched Porobot#7556] display a deck and its cards by specifying the `/deck` command and inserting the deck code as the `code` parameter:
//!
//! ```text
//! /deck code:CIBQCAICAQAQGBQIBEBAMBAJBMGBUHJNGE4AEAIBAIYQEAQGEU2QCAIBAIUQ
//! ```
//!
//! #### Named decks
//!
//! Optionally, you may add a name to your deck, which will be displayed above the deck code:
//!
//! ```text
//! /deck code:CIBQCAICAQAQGBQIBEBAMBAJBMGBUHJNGE4AEAIBAIYQEAQGEU2QCAIBAIUQ name:Gimbo's Depths
//! ```
//!
//! ### Permissions
//!
//! You can configure the bot's permissions by using Discord's Command Permissions system!
//!
//! You can access it via `SERVER NAME` → `Server Settings` → `Integrations` → [Patched Porobot#7556] `Manage`.
//!
//! See the following flowchart to understand how Command Permissions work:
//!
//! * [Flowchart](https://cdn.discordapp.com/attachments/697138785317814292/1042878162901672048/flowchart-for-new-permissions.png)
//!
//! [Patched Porobot#7556]: https://discord.com/api/oauth2/authorize?client_id=1071989978743193672&scope=applications.commands

#![doc(html_logo_url = "https://raw.githubusercontent.com/Steffo99/patched-porobot/main/icon.png")]

#[doc(hidden)]
#[tokio::main]
async fn main() {
    patched_porobot::discord::main::main().await;
}
