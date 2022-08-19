//! Module defining the [`Deck`] struct and its serialization methods and results.

use std::collections::HashMap;
use std::io::{Cursor, Read, Write};
use itertools::Itertools;
use varint_rs::{VarintReader, VarintWriter};
use crate::data::deckcode::version::{DeckCodeVersion, DeckCodeVersioned};
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
    /// Decode a deck code into a [`Vec`] of [bytes](u8).
    fn decode_code(code: &str) -> DeckDecodingResult<Vec<u8>> {
        data_encoding::BASE32_NOPAD
            .decode(code.as_bytes())
            .map_err(DeckDecodingError::Base32Encoding)
    }

    /// Encode a slice of [bytes](u8) into a deck code.
    fn encode_code(bytes: &[u8]) -> String {
        data_encoding::BASE32_NOPAD
            .encode(&bytes)
    }

    /// [Read] the header byte into a [format](DeckCodeFormat) and [version](DeckCodeVersion) tuple.
    fn read_header<R: Read>(reader: &mut R) -> DeckDecodingResult<(DeckCodeFormat, DeckCodeVersion)> {
        let mut format_version: [u8; 1] = [0; 1];
        reader.read_exact(&mut format_version).map_err(DeckDecodingError::Read)?;

        let format = DeckCodeFormat::try_from(format_version[0] >> 4).map_err(|_| DeckDecodingError::UnknownFormat)?;
        let version = DeckCodeVersion::try_from(format_version[0] & 0xF).map_err(|_| DeckDecodingError::UnknownVersion)?;

        Ok((format, version))
    }

    /// [Write] the header byte with the given [format](DeckCodeFormat) and [version](DeckCodeVersion).
    fn write_header<W: Write>(writer: &mut W, format: DeckCodeFormat, version: DeckCodeVersion) -> DeckEncodingResult<()> {
        let format: u8 = format.into();
        let version: u8 = version.into();

        let byte: u8 = (format << 4) | version;

        writer.write(&[byte]).map_err(DeckEncodingError::Write)?;

        Ok(())
    }

    /// [Read] a [`Deck`] using the [`F1`](DeckCodeFormat::F1) format.
    fn read_f1_body<R: Read>(reader: &mut R) -> DeckDecodingResult<Self> {
        // Create the deck's cards container
        let mut contents = HashMap::<CardCode, u32>::new();

        // Read the standard body
        for quantity in (1..=3).rev() {
            Self::read_f1_supergroup(reader, &mut contents, quantity)?;
        }

        // Read the extra body
        Self::read_f1_extra(reader, &mut contents)?;

        // Create and return the deck
        Ok(Deck { contents })
    }

    /// [Write] this [`Deck`] using the [`F1`](DeckCodeFormat::F1) format.
    fn write_f1_body<W: Write>(&self, writer: &mut W) -> DeckEncodingResult<()> {
        // Create four vecs of cards, for 3×, 2×, 1× and any× respectively.
        let mut triplets = Vec::<&CardCode>::new();
        let mut twins = Vec::<&CardCode>::new();
        let mut singletons = Vec::<&CardCode>::new();
        let mut extra = Vec::<(&CardCode, u32)>::new();

        // Fill the previous vecs
        for (code, quantity) in self.contents.iter() {
            match quantity {
                3 => triplets.push(code),
                2 => twins.push(code),
                1 => singletons.push(code),
                _ => extra.push((code, *quantity)),
            }
        }

        // Write the standard body
        Self::write_f1_supergroup(writer, &triplets)?;
        Self::write_f1_supergroup(writer, &twins)?;
        Self::write_f1_supergroup(writer, &singletons)?;

        // Write the extra body
        Self::write_f1_extra(writer, extra)?;

        Ok(())
    }

    /// [Read] the **groups** of a single supergroup.
    fn read_f1_supergroup<R: Read>(reader: &mut R, contents: &mut HashMap<CardCode, u32>, quantity: u32) -> DeckDecodingResult<()> {
        // Read the number of groups in the supergroup
        let len = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        // Read all groups
        for _ in 0..len {
            Self::read_f1_group(reader, contents, quantity)?;
        }

        Ok(())
    }

    /// Given a slice of [`CardCode`]s, group them by set and region.
    fn f1_group_cards<'cc>(codes: &[&'cc CardCode]) -> HashMap<(&'cc str, &'cc str), Vec<&'cc CardCode>> {
        // Create the hashmap accumulating groups of cards
        // It has the tuple (set, region) as key
        let mut groups = HashMap::<(&str, &str), Vec<&CardCode>>::new();

        // Insert all cards into the various groups
        for card in codes {
            // Determine the key tuple
            let key = (card.set(), card.region());

            // Create or insert groups
            match groups.get(&key) {
                None => {
                    let mut group = Vec::new();
                    group.push(*card);
                    groups.insert(key, group);
                }
                Some(_) => {
                    // FIXME: there's a better version for sure
                    let mut group = groups.remove(&key).unwrap();
                    group.push(*card);
                    groups.insert(key, group);
                }
            }
        }

        groups
    }

    /// [Write] the **groups** of a single supergroup.
    fn write_f1_supergroup<W: Write>(writer: &mut W, supergroup: &[&CardCode]) -> DeckEncodingResult<()> {
        // Arrange cards into groups
        let groups = Self::f1_group_cards(supergroup);

        // Determine the number of groups in the supergroup
        let len: u32 = groups.len().try_into().expect("groups length to be smaller than usize");
        writer.write_u32_varint(len).map_err(DeckEncodingError::Write)?;

        // Sort first by ascending group length, then by key
        let groups = groups
            .into_iter()
            .sorted_by(|a, b| a.1.len().cmp(&b.1.len()).then(a.0.cmp(&b.0)));

        // Write all groups
        for ((set, region), group) in groups {
            Self::write_f1_group(writer, &group, set, region)?;
        }

        Ok(())
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
    fn write_f1_group<W: Write>(writer: &mut W, group: &[&CardCode], set: &str, region: &str) -> DeckEncodingResult<()> {
        let len: u32 = group.len().try_into().expect("cards length to be smaller than usize");
        writer.write_u32_varint(len).map_err(DeckEncodingError::Write)?;

        let set: u32 = CardSet::from_code(set).try_into().map_err(|_| DeckEncodingError::UnknownSet)?;
        writer.write_u32_varint(set).map_err(DeckEncodingError::Write)?;

        let region: u32 = CardRegion::from_code(region).try_into().map_err(|_| DeckEncodingError::UnknownRegion)?;
        writer.write_u32_varint(region).map_err(DeckEncodingError::Write)?;

        for card in group {
            Self::write_f1_standard_card(writer, card.card())?;
        }

        Ok(())
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

        // While the cursor has still some bytes left...
        while let Ok(_) = Self::read_f1_extra_card(reader, contents) {}

        Ok(())
    }

    /// [Write] the **extra segment** of the deck code.
    fn write_f1_extra<W: Write>(writer: &mut W, codes: Vec<(&CardCode, u32)>) -> DeckEncodingResult<()> {
        for (code, quantity) in codes.iter().sorted() {
            Self::write_f1_extra_card(writer, code, *quantity)?;
        }

        Ok(())
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
    fn write_f1_extra_card<W: Write>(writer: &mut W, code: &CardCode, quantity: u32) -> DeckEncodingResult<()> {
        writer.write_u32_varint(quantity).map_err(DeckEncodingError::Write)?;

        let set = CardSet::from_code(code.set());
        let set: u32 = set.try_into().map_err(|_| DeckEncodingError::UnknownSet)?;
        writer.write_u32_varint(set).map_err(DeckEncodingError::Write)?;

        let region = CardRegion::from_code(code.region());
        let region: u32 = region.try_into().map_err(|_| DeckEncodingError::UnknownRegion)?;
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
            DeckCodeFormat::F1 => self.write_f1_body(&mut cursor)?,
        }

        Ok(Self::encode_code(&cursor.into_inner()))
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

    #[test]
    fn test_deser() {
        let deck = Deck::from_code("CICACAYGBAAQIBIBAMAQKKZPGEDQEBQEBEGBEFA2EYAQCAYGCABACAIFGUAQGBIH").unwrap();
        let code = deck.to_code(DeckCodeFormat::F1).unwrap();
        assert_eq!(code, "CICACAYGBAAQIBIBAMAQKKZPGEDQEBQEBEGBEFA2EYAQCAYGCABACAIFGUAQGBIH")
    }
}