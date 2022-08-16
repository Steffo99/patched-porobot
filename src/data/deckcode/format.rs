//! This module defines [`DeckCodeFormat`].


/// The format of a deck code.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeckCodeFormat {
    /// The only format specified so far.
    F1 = 1,
}
