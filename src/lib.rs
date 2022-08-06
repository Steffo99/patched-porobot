//! Crate providing parsing, indexing, and displaying utilities for Legends of Runeterra data files.
//!
//! # Features
//!
//! - `search`: Adds a search engine based on [tantivy] for Legends of Runeterra data.
//! - `telegram`: Adds a [Telegram bot](https://core.telegram.org/bots/api) based on [teloxide] for Legends of Runeterra data.
//!
//! # Legal
//!
//! [patched_porobot](self) isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties. Riot Games, and all associated properties are trademarks or registered trademarks of Riot Games, Inc.

#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/Steffo99/patched-porobot/main/icon.png")]

pub mod data;

// #[cfg(feature = "search")]
// pub mod search;

// #[cfg(feature = "telegram")]
// pub mod telegram;
