//! Module defining the [`Deck`] struct and its serialization methods.

use sorted_vec::SortedVec;
use super::quantity::CardCodeQuantity;
use super::format::DeckCodeFormat;


/// A unshuffled Legends of Runeterra card deck.
pub struct Deck {
    /// The contents of the deck, represented as a [`SortedVec`] of [`CardCodeQuantity`].
    pub contents: SortedVec<CardCodeQuantity>,
}


impl Deck {
    /// Deserialize a deck code into a [`Deck`].
    pub fn from_code(code: &str) -> Self {
        todo!()
    }

    /// Serialize the [`Deck`] into a deck code of the given [format](DeckCodeFormat).
    pub fn to_code(self, version: DeckCodeFormat) -> String {
        match version {
            DeckCodeFormat::F1 => self.to_f1_code(),
        }
    }

    /// Serialize the [`Deck`] into a deck code of [`DeckCodeFormat::F1`].
    fn to_f1_code(self) -> String {
        todo!()
    }
}