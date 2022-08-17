//! Module defining the [`Deck`] struct and its serialization methods and results.

use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use varint_rs::{VarintReader, VarintWriter};
use crate::data::deckcode::version::{DeckCodeVersion, DeckCodeVersioned};
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
    /// Decode a deck code into a [`Vec`] of [`u8`].
    fn decode_code(code: &str) -> DeckDecodingResult<Vec<u8>> {
        data_encoding::BASE32_NOPAD
            .decode(code.as_bytes())
            .map_err(DeckDecodingError::Base32Encoding)
    }

    /// Encode a [`Vec`] of [`u8`] into a deck code.
    fn encode_code(bytes: Vec<u8>) -> String {
        data_encoding::BASE32_NOPAD
            .encode(&bytes)
    }

    /// Read the format and version byte from a readable stream of bytes.
    fn read_header<R: Read>(reader: &mut R) -> DeckDecodingResult<(DeckCodeFormat, DeckCodeVersion)> {
        let mut format_version: [u8; 1] = [0; 1];
        reader.read_exact(&mut format_version).map_err(DeckDecodingError::Read)?;

        let format = DeckCodeFormat::try_from(format_version[0] >> 4).map_err(|_| DeckDecodingError::UnknownFormat)?;
        let version = DeckCodeVersion::try_from(format_version[0] & 0xF).map_err(|_| DeckDecodingError::UnknownVersion)?;

        Ok((format, version))
    }

    /// Write the given format and version into a writable stream of bytes.
    fn write_header<W: Write>(writer: &mut W, format: DeckCodeFormat, version: DeckCodeVersion) -> DeckEncodingResult<usize> {
        let format: u8 = format.into();
        let version: u8 = version.into();

        let byte: u8 = (format << 4) | version;

        writer.write(&[byte]).map_err(DeckEncodingError::Write)
    }

    /// Read [the triplets, twins, and singletons supergroups](Self::read_supergroup) into the given hashmap.
    fn read_main<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>) -> DeckDecodingResult<()> {
        for quantity in (1..=3).rev() {
            Self::read_supergroup(reader, contents, quantity)?;
        }

        Ok(())
    }

    /// Read [the groups](Self::read_group) of [a single supergroup](Self::read_main) into the given hashmap.
    fn read_supergroup<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>, quantity: u32) -> DeckDecodingResult<()> {
        let group_count = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        for _group in 0..group_count {
            Self::read_group(reader, contents, quantity)?;
        }

        Ok(())
    }

    /// Read [the card codes](Self::read_card_main) of [a single group](Self::read_supergroup) into the given hashmap.
    fn read_group<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>, quantity: u32) -> DeckDecodingResult<()> {
        let card_count = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        let set = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;
        let set = CardSet::from(set).to_code().ok_or(DeckDecodingError::UnknownSet)?;

        let region = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;
        let region = CardRegion::from(region).to_code().ok_or(DeckDecodingError::UnknownRegion)?;

        for _card in 0..card_count {
            Self::read_card_main(reader, contents, quantity, &set, &region)?;
        }

        Ok(())
    }

    /// Read [a single card code](Self::read_group) into the given hashmap.
    fn read_card_main<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>, quantity: u32, set: &str, region: &str) -> DeckDecodingResult<()> {
        let card = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        let code = CardCode::from_s_r_c(set, region, card);
        contents.insert(code, quantity);

        Ok(())
    }

    /// Write [a single card number](Self::write_group) into the given writer.
    fn write_card_main<W: Write>(writer: &mut W, card: &str) -> DeckEncodingResult<()> {
        let card = card.parse::<u32>().map_err(DeckEncodingError::InvalidCardNumber)?;
        writer.write_u32_varint(card).map_err(DeckEncodingError::Write)?;

        Ok(())
    }

    /// Read [all possible card codes with a non-standard quantity](Self::read_card_extra) into the given hashmap.
    fn read_extra<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>) -> DeckDecodingResult<()> {
        let len = cursor.get_ref().len();

        // While the cursor has still some bytes left...
        while (cursor.position() as usize) < (len - 1) {
            Self::read_card_extra(reader, contents)?;
        }

        Ok(())
    }

    /// Read [a single card code with a non-standard quantity](Self::read_extra) into the given hashmap.
    fn read_card_extra<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>) -> DeckDecodingResult<()> {
        let quantity = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        let set = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;
        let set = CardSet::from(set).to_code().ok_or(DeckDecodingError::UnknownSet)?;

        let region = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;
        let region = CardRegion::from(region).to_code().ok_or(DeckDecodingError::UnknownRegion)?;

        let card = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        let code = CardCode::from_s_r_c(&set, &region, card);
        contents.insert(code, quantity);

        Ok(())
    }

    /// Write [a single card code with a non-standard quantity](Self::write_extra) into the given writer.
    fn write_card_extra<W: Write>(writer: &mut W, code: CardCode, quantity: u32) -> DeckEncodingResult<()> {
        writer.write_u32_varint(quantity).map_err(DeckEncodingError::Write)?;

        let set = CardSet::try_from(code.set()).map_err(|_| DeckEncodingError::UnknownSet)?;
        let set: u32 = set.into();
        writer.write_u32_varint(set).map_err(DeckEncodingError::Write)?;

        let region = CardRegion::try_from(code.region()).map_err(|_| DeckEncodingError::UnknownRegion)?;
        let region: u32 = region.into();
        writer.write_u32_varint(region).map_err(DeckEncodingError::Write)?;

        let card = code.card().parse::<u32>().map_err(DeckEncodingError::InvalidCardNumber)?;
        writer.write_u32_varint(card).map_err(DeckEncodingError::Write)?;

        Ok(())
    }

    /// Deserialize a deck code into a [`Deck`].
    pub fn from_code(code: &str) -> DeckDecodingResult<Deck> {
        let mut cursor = Cursor::new(Self::decode_code(&code)?);

        let (format, _version) = Self::read_header(&mut cursor)?;

        let mut contents = HashMap::<CardCode, u32>::new();

        match format {
            DeckCodeFormat::F1 => {
                Self::read_main(&mut cursor, &mut contents)?;
                Self::read_extra(&mut cursor, &mut contents)?;
                Ok(Deck { contents })
            }
        }
    }

    /// Serialize the [`Deck`] into a deck code of the given [format](DeckCodeFormat).
    pub fn to_code(&self, format: DeckCodeFormat) -> DeckEncodingResult<String> {
        let mut cursor = Cursor::new(Vec::new());

        let version = self.min_deckcode_version().ok_or(DeckEncodingError::UnknownVersion)?;

        Self::write_header(&mut cursor, format, version)?;

        match format {
            DeckCodeFormat::F1 => {
                todo!()
            }
        }

        Ok(Self::encode_code(cursor.into_inner()))
    }
}


/// An error occoured while decoding a [`Deck`] from a code.
#[derive(Debug)]
pub enum DeckDecodingError {
    /// The provided string was not a valid base-32 string.
    Base32Encoding(data_encoding::DecodeError),
    /// The decoder cursor was not able to read data from the byte buffer.
    Read(std::io::Error),
    /// The deck code format of the provided string was unknown.
    UnknownFormat,
    /// The deck code version of the provided string was unknown.
    UnknownVersion,
    /// The deck code contains a set with an unknown short code.
    UnknownSet,
    /// The deck code contains a region with an unknown short code.
    UnknownRegion,
}

/// An error occoured while encoding a [`Deck`] into a code.
#[derive(Debug)]
pub enum DeckEncodingError {
    /// The encoder cursor was not able to write data into the byte buffer.
    Write(std::io::Error),
    /// The deck code version of the deck could not be determined.
    UnknownVersion,
    /// A card in the deck belongs to a set with an unknown internal id.
    UnknownSet,
    /// A card in the deck belongs to a region with an unknown internal id.
    UnknownRegion,
    /// A card in the deck has a invalid card number segment in the card code.
    InvalidCardNumber(std::num::ParseIntError),
}

pub type DeckDecodingResult<T> = Result<T, DeckDecodingError>;
pub type DeckEncodingResult<T> = Result<T, DeckEncodingError>;


#[cfg(test)]
mod tests {
    use super::*;

    // TODO: TEST THIS!
}