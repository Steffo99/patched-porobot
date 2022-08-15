//! Module defining the types used to deserialize deck codes.
//! 
//! Adapted from [RiotGames/LoRDeckCodes](https://github.com/RiotGames/LoRDeckCodes) and from [iulianR/lordeckcodes-rs](https://github.com/iulianR/lordeckcodes-rs/blob/master/src/lib.rs).

use crate::data::setbundle::card::Card;


/// A deck code, as defined in the [deck codes specification](https://github.com/RiotGames/LoRDeckCodes).
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DeckCode {
    /// The deck code as a string.
    pub full: String
}


impl From<String> for DeckCode {
    fn from(full: String) -> Self {
        Self { full }
    }
}


impl From<DeckCode> for String {
    fn from(dc: DeckCode) -> Self {
        dc.full
    }
}


impl From<Vec<(Card, usize)>> for DeckCode {
    fn from(_: Vec<(Card, usize)>) -> Self {
        todo!()
    }
}


impl From<DeckCode> for Vec<(Card, usize)> {
    fn from(_: DeckCode) -> Self {
        todo!()
    }
}