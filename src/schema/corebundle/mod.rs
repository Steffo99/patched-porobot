//! This module defines the types used in Data Dragon's [Core Bundle](https://developer.riotgames.com/docs/lor#data-dragon_core-bundles) `globals.json` files.

mod globals;
mod vocabterm;
mod keyword;
mod region;
mod speed;
mod rarity;
mod set;

pub use globals::CoreGlobals;
pub use vocabterm::CoreVocabTerm;
pub use keyword::CoreKeyword;
pub use region::CoreRegion;
pub use speed::CoreSpellSpeed;
pub use rarity::CoreRarity;
pub use set::CoreSet;
