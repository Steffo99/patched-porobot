//! This module defines the types used in Data Dragon's [Core Bundle](https://developer.riotgames.com/docs/lor#data-dragon_core-bundles) `globals.json` files.

pub use globals::CoreGlobals;
pub use keyword::CoreKeyword;
pub use rarity::CoreRarity;
pub use region::CoreRegion;
pub use set::CoreSet;
pub use speed::CoreSpellSpeed;
pub use vocabterm::CoreVocabTerm;

mod globals;
mod vocabterm;
mod keyword;
mod region;
mod speed;
mod rarity;
mod set;

