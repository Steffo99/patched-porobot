//! This module defines the [Card] type and the types used in it to better describe attributes.

mod card;
mod art;
mod r#type;
mod rarity;
mod region;
mod set;
mod speed;
mod keyword;

pub use card::Card;
pub use art::CardArt;
pub use r#type::CardType;
pub use rarity::CardRarity;
pub use region::CardRegion;
pub use set::CardSet;
pub use speed::SpellSpeed;
pub use keyword::CardKeyword;
