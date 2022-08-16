//! Module defining the types used to deserialize deck codes.
//! 
//! Adapted from [RiotGames/LoRDeckCodes](https://github.com/RiotGames/LoRDeckCodes) and from [iulianR/lordeckcodes-rs](https://github.com/iulianR/lordeckcodes-rs/).

use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use crate::data::setbundle::code::CardCode;


pub struct Deck {
    pub contents: Vec<(CardCode, usize)>,
}


impl Deck {
    /// Compare two deck cards for ordering the [contents](Deck::contents) [Vec].
    fn compare_tuples(a: (CardCode, usize), b: (CardCode, usize)) -> Ordering {
        // Why isn't partial_eq public...?
        let ord = a.1.cmp(&b.1);
        if ord != Ordering::Equal { return ord; };
        let ord = a.0.set().cmp(b.0.set());
        if ord != Ordering::Equal { return ord; };
        let ord = a.0.region().cmp(b.0.region());
        if ord != Ordering::Equal { return ord; };
        let ord = a.0.card().cmp(b.0.card());
        if ord != Ordering::Equal { return ord; };
        a.0.token().cmp(b.0.token())
    }

    pub fn from_code(code: String) -> Self {
        todo!()
    }

    pub fn to_code(&self) -> String {
        let hm: HashMap<usize, HashMap<String, HashMap<String, &CardCode>>> = HashMap::new();

        // self.contents.binary_search_by(compare_tuples)

        todo!()
    }
}