//! Module defining the [`Deck`] struct and its serialization methods and results.

use std::collections::HashMap;
use std::io::{Cursor, Read};
use varint_rs::VarintReader;
use crate::data::deckcode::version::DeckCodeVersion;
use crate::data::setbundle::card::Card;
use crate::data::setbundle::code::CardCode;
use crate::data::setbundle::region::CardRegion;
use crate::data::setbundle::set::CardSet;
use super::format::DeckCodeFormat;


/// A unshuffled Legends of Runeterra card deck.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deck {
    /// The contents of the deck, represented as a [`HashMap`] mapping [`CardCode`]s to the number of inserted cards.
    pub contents: HashMap<CardCode, u32>,
}


impl Deck {
    /// Deserialize a deck code into a [`Deck`].
    pub fn from_code(code: &str) -> DeckDecodingResult {
        // Create a Vec of bytes from the input deck code
        let bytes = data_encoding::BASE32_NOPAD.decode(code.as_bytes())
            .map_err(DeckDecodingError::Base32)?;

        // Create a cursor spanning the bytes, so they can be read
        let mut cursor = Cursor::new(&bytes);

        // Find the format and the version of the deck code
        let mut format_version: [u8; 1] = [0; 1];
        cursor.read_exact(&mut format_version).map_err(DeckDecodingError::Read)?;
        let format = DeckCodeFormat::try_from(format_version[0] >> 4).map_err(|_| DeckDecodingError::UnknownFormat)?;
        let _version = DeckCodeVersion::try_from(format_version[0] & 0xF).map_err(|_| DeckDecodingError::UnknownVersion)?;

        // Create the hashmap containing the deck's cards
        let mut contents = HashMap::<CardCode, u32>::new();

        // Do something different based on the format of the deck code
        match format {
            DeckCodeFormat::F1 => {
                // All deck codes have three supergroups, containing cards inserted 3×, 2×, and 1× respectively
                for quantity in (1..=3).rev() {
                    // Supergroups contain a group for each set and region combination of the cards inside
                    let group_count = cursor.read_u32_varint().map_err(DeckDecodingError::Read)?;

                    for _group_number in 0..group_count {
                        // Groups may contain any number of cards
                        let card_count = cursor.read_u32_varint().map_err(DeckDecodingError::Read)?;

                        // Sets are represented via their internal id and must be converted to strings
                        let set = cursor.read_u32_varint().map_err(DeckDecodingError::Read)?;
                        let set = CardSet::from_id(set).to_code().expect("sets with an internal id to have a short code");

                        // Regions are represented via their internal id and must be converted to strings
                        let region = cursor.read_u32_varint().map_err(DeckDecodingError::Read)?;
                        let region = CardRegion::from_id(region).to_code().expect("regions with an internal id to have a short code");

                        for _card_number in 0..card_count {
                            // Card numbers are represented "directly"
                            let card = cursor.read_u32_varint().map_err(DeckDecodingError::Read)?;

                            // Create a card code from the obtained information
                            let code = format!("{:02}{}{:03}", &set, &region, &card);
                            let code = CardCode::from(code);

                            contents.insert(code, quantity);
                        }
                    }
                }

                // Read cards with a quantity of 4 or more. Wild!
                // No groups here, just raw cards
                let len = cursor.get_ref().len();
                while (cursor.position() as usize) < (len - 1) {
                    let quantity = cursor.read_u32_varint().map_err(DeckDecodingError::Read)?;

                    let set = cursor.read_u32_varint().map_err(DeckDecodingError::Read)?;
                    let set = CardSet::from_id(set).to_code().expect("sets with an internal id to have a short code");

                    let region = cursor.read_u32_varint().map_err(DeckDecodingError::Read)?;
                    let region = CardRegion::from_id(region).to_code().expect("regions with an internal id to have a short code");

                    let card = cursor.read_u32_varint().map_err(DeckDecodingError::Read)?;

                    let code = format!("{:02}{}{:03}", &set, &region, &card);
                    let code = CardCode::from(code);

                    contents.insert(code, quantity);
                }

                Ok(Deck { contents })
            }
        }
    }

    /// Serialize the [`Deck`] into a deck code of the given [format](DeckCodeFormat).
    pub fn to_code(self, format: DeckCodeFormat) -> String {
        match format {
            DeckCodeFormat::F1 => todo!(),
        }
    }
}


/// An error occoured while decoding a [`Deck`](crate::data::deckcode::deck::Deck) from a code.
#[derive(Debug)]
pub enum DeckDecodingError {
    /// The provided string was not a valid base-32 string.
    Base32(data_encoding::DecodeError),
    /// The provided string was probably not long enough to be a valid deck code.
    Read(std::io::Error),
    /// The deck code format of the provided string was unknown.
    UnknownFormat,
    /// The deck code version of the provided string was unknown.
    UnknownVersion,
}


pub type DeckDecodingResult = Result<Deck, DeckDecodingError>;


#[cfg(test)]
mod tests {
    use super::*;

    // TODO: TEST THIS!
}