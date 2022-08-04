//! This module defines the types used in Data Dragon's [Set Bundle](https://developer.riotgames.com/docs/lor#data-dragon_set-bundles) `set.json` files.

pub use art::CardArt;
pub use card::Card;
pub use keyword::CardKeyword;
pub use r#type::CardType;
pub use rarity::CardRarity;
pub use region::CardRegion;
pub use set::CardSet;
pub use speed::SpellSpeed;

mod card;
mod art;
mod r#type;
mod rarity;
mod region;
mod set;
mod speed;
mod keyword;

