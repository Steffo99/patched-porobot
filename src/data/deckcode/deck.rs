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

    /// [Read] a [`Deck`] using the [`F1`](DeckCodeFormat::F1) format.
    fn read_f1_body<R: Read>(reader: &mut R) -> DeckDecodingResult<Self> {
        let mut contents = HashMap::<CardCode, u32>::new();
        Self::read_f1_standard(reader, &mut contents)?;
        Self::read_f1_extra(reader, &mut contents)?;
        Ok(Deck { contents })
    }

    /// [Write] this [`Deck`] using the [`F1`](DeckCodeFormat::F1) format.
    fn write_f1_body<W: Write>(&self, writer: &mut W) -> DeckEncodingResult<()> {
        let mut supergroups = HashMap::<u32, Vec<CardCode>>::new();

        for (card, quantity) in self.contents.iter() {
            match supergroups.get(quantity) {
                None => {
                    let mut supergroup = Vec::<CardCode>::new();
                    supergroup.push(card.to_owned());
                    supergroups.insert(*quantity, supergroup);
                }
                Some(mut supergroup) => {
                    supergroup.push(card.to_owned());
                }
            }
        }

        ///
        for quantity in (1..=3).rev() {
            match supergroups.get(&quantity) {
                None => {}
                Some(supergroup) => {
                    Self::write_f1_supergroup(writer, supergroup, quantity)?;
                    supergroups.remove(&quantity);
                }
            }
        };
        for (quantity, supergroup) in supergroups.iter() {

        }

        Ok(())
    }

    /// [Read] the triplets, twins, and singletons **supergroups**.
    fn read_f1_standard<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>) -> DeckDecodingResult<()> {
        for quantity in (1..=3).rev() {
            Self::read_f1_supergroup(reader, contents, quantity)?;
        }

        Ok(())
    }

    /// [Write] the **standard segment** of the deck code.
    fn write_f1_standard<W: Write>(writer: &mut W, triplets: Vec<CardCode>, twins: Vec<CardCode>, singletons: Vec<CardCode>) -> DeckEncodingResult<()> {
        Self::write_f1_supergroup(writer, triplets, 3)?;
        Self::write_f1_supergroup(writer, twins, 2)?;
        Self::write_f1_supergroup(writer, singletons, 1)?;

        Ok(())
    }

    /// [Read] the **groups** of a single supergroup.
    fn read_f1_supergroup<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>, quantity: u32) -> DeckDecodingResult<()> {
        let group_count = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        for _group in 0..group_count {
            Self::read_f1_group(reader, contents, quantity)?;
        }

        Ok(())
    }

    /// [Write] the **groups** of a single supergroup.
    fn write_f1_supergroup<W: Write>(writer: &mut W, supergroup: &Vec<CardCode>, quantity: u32) -> DeckEncodingResult<()> {
        todo!()
    }

    /// [Read] the **cards** of a single group.
    fn read_f1_group<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>, quantity: u32) -> DeckDecodingResult<()> {
        let card_count = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        let set = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;
        let set = CardSet::from(set).to_code().ok_or(DeckDecodingError::UnknownSet)?;

        let region = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;
        let region = CardRegion::from(region).to_code().ok_or(DeckDecodingError::UnknownRegion)?;

        for _card in 0..card_count {
            Self::read_f1_standard_card(reader, contents, quantity, &set, &region)?;
        }

        Ok(())
    }

    /// [Write] the **cards** of a single group.
    fn write_f1_group<W: Write>(writer: &mut W, group: Vec<CardCode>, set: u32, region: u32) -> DeckDecodingResult<()> {
        todo!()
    }

    /// [Read] **a single card**.
    fn read_f1_standard_card<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>, quantity: u32, set: &str, region: &str) -> DeckDecodingResult<()> {
        let card = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        let code = CardCode::from_s_r_c(set, region, card);
        contents.insert(code, quantity);

        Ok(())
    }

    /// [Write] **a single card**.
    fn write_f1_standard_card<W: Write>(writer: &mut W, card: &str) -> DeckEncodingResult<()> {
        let card = card.parse::<u32>().map_err(DeckEncodingError::InvalidCardNumber)?;
        writer.write_u32_varint(card).map_err(DeckEncodingError::Write)?;

        Ok(())
    }

    /// [Read] the **extra segment** of the deck code.
    fn read_f1_extra<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>) -> DeckDecodingResult<()> {
        let len = cursor.get_ref().len();

        // While the cursor has still some bytes left...
        while (cursor.position() as usize) < (len - 1) {
            Self::read_f1_extra_card(reader, contents)?;
        }

        Ok(())
    }

    /// [Write] the **extra segment** of the deck code.
    fn write_f1_extra<W: Write>(writer: &mut W, codes: Vec<CardCode>) -> DeckDecodingResult<()> {
        todo!()
    }

    /// [Read] **a single card** with a **non-standard quantity**.
    fn read_f1_extra_card<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>) -> DeckDecodingResult<()> {
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

    /// [Write] **a single card** with a **non-standard quantity**.
    fn write_f1_card_extra<W: Write>(writer: &mut W, code: CardCode, quantity: u32) -> DeckEncodingResult<()> {
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

        match format {
            DeckCodeFormat::F1 => Self::read_f1_body(&mut cursor)
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