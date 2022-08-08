//! Crate providing parsing, indexing, and displaying utilities for Legends of Runeterra data files.
//!
//! # Usage
//!
//! This is the technical documentation of the [`patched_porobot`](self) Rust crate.
//!
//! If you are looking for the documentation of its implementations, please visit one of the following pages:
//!
//! - [Usage of the Telegram bot](../patched_porobot_telegram)
//! - ~~[Usage of the Discord bot](../patched_porobot_discord)~~
//! - ~~[Usage of the Matrix bot](../patched_porobot_matrix)~~
//!
//! # Features
//!
//! [`patched_porobot`](self) supports conditional compilation via [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html).
//!
//! While the [`data`] module is always included, [the other modules](#modules) provided by this crate may be used by selecting the features of the same name.
//!
//! ## Binaries
//!
//! Additionally, every one of the following features enables the compilation of an additional binary target:
//!
//! - [`telegram`] enables the compilation of `patched_porobot_telegram`, a [Telegram inline bot](https://core.telegram.org/bots/api) allowing users to search and send cards in any Telegram chat;
//! - ~~[`discord`] enables the compilation of `patched_porobot_discord`, a [Discord bot](https://discord.com/developers/docs/intro#bots-and-apps) allowing Discord servers the bot is added to to search and send cards in their channels~~;
//! - ~~[`matrix`] enables the compilation of `patched_porobot_matrix`, a Matrix bot parsing messages in the rooms where it is added to to send details about the cards mentioned in messages~~.
//!
//! # Legal
//!
//! [`patched_porobot`](self) isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing Riot Games properties. Riot Games, and all associated properties are trademarks or registered trademarks of Riot Games, Inc.

#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/Steffo99/patched-porobot/main/icon.png")]

pub mod data;

#[cfg(feature = "search")]
pub mod search;

#[cfg(feature = "telegram")]
pub mod telegram;

#[cfg(feature = "discord")]
pub mod discord;

#[cfg(feature = "matrix")]
pub mod matrix;
