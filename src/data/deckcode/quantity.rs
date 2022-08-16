//! Module defining the [`CardQuantity`] struct.

use crate::data::setbundle::code::CardCode;


/// A card placed in a [`Deck`](super::deck::Deck), which may be repeated multiple times.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CardCodeQuantity {
    /// The number of inserted copies of the card.
    pub qty: u8,

    /// The code of the inserted card.
    pub code: CardCode,
}
