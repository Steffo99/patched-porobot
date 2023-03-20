//! Module defining the [`Deck`] struct and its serialization methods and results.

use super::format::DeckCodeFormat;
use crate::data::deckcode::version::{DeckCodeVersion, DeckCodeVersioned};
use crate::data::setbundle::card::{Card, CardIndex};
use crate::data::setbundle::code::CardCode;
use crate::data::setbundle::region::CardRegion;
use crate::data::setbundle::set::CardSet;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{Cursor, Read, Write};
use varint_rs::{VarintReader, VarintWriter};
use crate::data::setbundle::supertype::CardSupertype;

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
        data_encoding::BASE32_NOPAD.encode(bytes)
    }

    /// [Read] the header byte into a [format](DeckCodeFormat) and [version](DeckCodeVersion) tuple.
    fn read_header<R: Read>(
        reader: &mut R,
    ) -> DeckDecodingResult<(DeckCodeFormat, DeckCodeVersion)> {
        let mut format_version: [u8; 1] = [0; 1];
        reader
            .read_exact(&mut format_version)
            .map_err(DeckDecodingError::Read)?;

        let format = DeckCodeFormat::try_from(format_version[0] >> 4)
            .map_err(|_| DeckDecodingError::UnknownFormat)?;
        let version = DeckCodeVersion::try_from(format_version[0] & 0xF)
            .map_err(|_| DeckDecodingError::UnknownVersion)?;

        Ok((format, version))
    }

    /// [Write] the header byte with the given [format](DeckCodeFormat) and [version](DeckCodeVersion).
    fn write_header<W: Write>(
        writer: &mut W,
        format: DeckCodeFormat,
        version: DeckCodeVersion,
    ) -> DeckEncodingResult<()> {
        let format: u8 = format.into();
        let version: u8 = version.into();

        let byte: u8 = (format << 4) | (version & 0xF);

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
    fn read_f1_supergroup<R: Read>(
        reader: &mut R,
        contents: &mut HashMap<CardCode, u32>,
        quantity: u32,
    ) -> DeckDecodingResult<()> {
        // Read the number of groups in the supergroup
        let len = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        // Read all groups
        for _ in 0..len {
            Self::read_f1_group(reader, contents, quantity)?;
        }

        Ok(())
    }

    /// Given a slice of [`CardCode`]s, group them by set and region.
    fn f1_group_cards<'cc>(
        codes: &[&'cc CardCode],
    ) -> HashMap<(&'cc str, &'cc str), Vec<&'cc CardCode>> {
        // Create the hashmap accumulating groups of cards
        // It has the tuple (set, region) as key
        let mut groups = HashMap::new();

        // Insert all cards into the various groups
        for card in codes {
            // Determine the key tuple
            let key = (card.set(), card.region());

            // Create or insert groups
            match groups.get(&key) {
                None => {
                    let group = vec![*card];
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
    fn write_f1_supergroup<W: Write>(
        writer: &mut W,
        supergroup: &[&CardCode],
    ) -> DeckEncodingResult<()> {
        // Arrange cards into groups
        let groups = Self::f1_group_cards(supergroup);

        // Determine the number of groups in the supergroup
        let len: u32 = groups
            .len()
            .try_into()
            .expect("groups length to be smaller than usize");
        writer
            .write_u32_varint(len)
            .map_err(DeckEncodingError::Write)?;

        // Sort first by ascending group length, then by key
        let groups = groups
            .into_iter()
            .sorted_by(|(a_key, a_group), (b_key, b_group)| {
                a_group.len().cmp(&b_group.len()).then(a_key.cmp(b_key))
            });

        // Write all groups
        for ((set, region), group) in groups {
            Self::write_f1_group(writer, &group, set, region)?;
        }

        Ok(())
    }

    /// [Read] the **cards** of a single group.
    fn read_f1_group<R: Read>(
        reader: &mut R,
        contents: &mut HashMap<CardCode, u32>,
        quantity: u32,
    ) -> DeckDecodingResult<()> {
        let card_count = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        let set = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;
        let set = format!("{:02}", &set);

        let region = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;
        let region = CardRegion::from(region)
            .to_code()
            .ok_or(DeckDecodingError::UnknownRegion)?;

        for _card in 0..card_count {
            Self::read_f1_standard_card(reader, contents, quantity, &set, &region)?;
        }

        Ok(())
    }

    /// [Write] the **cards** of a single group.
    fn write_f1_group<W: Write>(
        writer: &mut W,
        group: &[&CardCode],
        set: &str,
        region: &str,
    ) -> DeckEncodingResult<()> {
        let len: u32 = group
            .len()
            .try_into()
            .expect("cards length to be smaller than usize");
        writer
            .write_u32_varint(len)
            .map_err(DeckEncodingError::Write)?;

        let set: u32 = set.parse()
            .map_err(|_| DeckEncodingError::UnknownSet)?;
        writer
            .write_u32_varint(set)
            .map_err(DeckEncodingError::Write)?;

        let region: u32 = CardRegion::from_code(region)
            .try_into()
            .map_err(|_| DeckEncodingError::UnknownRegion)?;
        writer
            .write_u32_varint(region)
            .map_err(DeckEncodingError::Write)?;

        for card in group.iter().sorted() {
            Self::write_f1_standard_card(writer, card.card())?;
        }

        Ok(())
    }

    /// [Read] **a single card**.
    fn read_f1_standard_card<R: Read>(
        reader: &mut R,
        contents: &mut HashMap<CardCode, u32>,
        quantity: u32,
        set: &str,
        region: &str,
    ) -> DeckDecodingResult<()> {
        let card = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        let code = CardCode::from_s_r_c(set, region, card);
        contents.insert(code, quantity);

        Ok(())
    }

    /// [Write] **a single card**.
    fn write_f1_standard_card<W: Write>(writer: &mut W, card: &str) -> DeckEncodingResult<()> {
        let card = card
            .parse::<u32>()
            .map_err(DeckEncodingError::InvalidCardNumber)?;
        writer
            .write_u32_varint(card)
            .map_err(DeckEncodingError::Write)?;

        Ok(())
    }

    /// [Read] the **extra segment** of the deck code.
    fn read_f1_extra<R: Read>(
        reader: &mut R,
        contents: &mut HashMap<CardCode, u32>,
    ) -> DeckDecodingResult<()> {
        // While the cursor has still some bytes left...
        while Self::read_f1_extra_card(reader, contents).is_ok() {}

        Ok(())
    }

    /// [Write] the **extra segment** of the deck code.
    fn write_f1_extra<W: Write>(
        writer: &mut W,
        codes: Vec<(&CardCode, u32)>,
    ) -> DeckEncodingResult<()> {
        for (code, quantity) in codes.iter().sorted() {
            Self::write_f1_extra_card(writer, code, *quantity)?;
        }

        Ok(())
    }

    /// [Read] **a single card** with a **non-standard quantity**.
    fn read_f1_extra_card<R: Read>(
        reader: &mut R,
        contents: &mut HashMap<CardCode, u32>,
    ) -> DeckDecodingResult<()> {
        let quantity = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        let set = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;
        let set = CardSet::from(set)
            .to_code()
            .ok_or(DeckDecodingError::UnknownSet)?;

        let region = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;
        let region = CardRegion::from(region)
            .to_code()
            .ok_or(DeckDecodingError::UnknownRegion)?;

        let card = reader.read_u32_varint().map_err(DeckDecodingError::Read)?;

        let code = CardCode::from_s_r_c(&set, &region, card);
        contents.insert(code, quantity);

        Ok(())
    }

    /// [Write] **a single card** with a **non-standard quantity**.
    fn write_f1_extra_card<W: Write>(
        writer: &mut W,
        code: &CardCode,
        quantity: u32,
    ) -> DeckEncodingResult<()> {
        writer
            .write_u32_varint(quantity)
            .map_err(DeckEncodingError::Write)?;

        let set = CardSet::from_code(code.set());
        let set: u32 = set.try_into().map_err(|_| DeckEncodingError::UnknownSet)?;
        writer
            .write_u32_varint(set)
            .map_err(DeckEncodingError::Write)?;

        let region = CardRegion::from_code(code.region());
        let region: u32 = region
            .try_into()
            .map_err(|_| DeckEncodingError::UnknownRegion)?;
        writer
            .write_u32_varint(region)
            .map_err(DeckEncodingError::Write)?;

        let card = code
            .card()
            .parse::<u32>()
            .map_err(DeckEncodingError::InvalidCardNumber)?;
        writer
            .write_u32_varint(card)
            .map_err(DeckEncodingError::Write)?;

        Ok(())
    }

    /// Deserialize a deck code into a [`Deck`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use patched_porobot::data::deckcode::deck::Deck;
    ///
    /// Deck::from_code("CQBQCBAJBUCAKCRYHKTADNIBAYBQSDQ2DQ3FEWACAECQVNQBAIBQSOK5AEAQGCIV")
    ///     .expect("deck to be deserialized successfully");
    /// ```
    pub fn from_code(code: &str) -> DeckDecodingResult<Deck> {
        let mut cursor = Cursor::new(Self::decode_code(code)?);

        let (format, _version) = Self::read_header(&mut cursor)?;

        match format {
            DeckCodeFormat::F1 => Self::read_f1_body(&mut cursor),
        }
    }

    /// Serialize the [`Deck`] into a deck code of the given [format](DeckCodeFormat).
    ///
    /// # Example
    ///
    /// ```rust
    /// use patched_porobot::deck;
    /// use patched_porobot::data::deckcode::deck::Deck;
    /// use patched_porobot::data::deckcode::format::DeckCodeFormat;
    ///
    /// let d: Deck = deck![
    ///     "01DE002": 40,
    /// ];
    ///
    /// d.to_code(DeckCodeFormat::F1).expect("deck to be serialized successfully");
    /// ```
    pub fn to_code(&self, format: DeckCodeFormat) -> DeckEncodingResult<String> {
        let mut cursor = Cursor::new(Vec::new());

        let version = self
            .min_deckcode_version()
            .ok_or(DeckEncodingError::UnknownVersion)?;

        Self::write_header(&mut cursor, format, version)?;

        match format {
            DeckCodeFormat::F1 => self.write_f1_body(&mut cursor)?,
        }

        Ok(Self::encode_code(&cursor.into_inner()))
    }

    /// Get an [`Iterator`] of all Champion [`Card`]s in the deck.
    pub fn champions<'a>(&'a self, cards: &'a CardIndex) -> impl Iterator<Item = &'a Card> {
        self.contents.keys()
            .filter_map(|cc| cc.to_card(cards))
            .filter(|c| c.supertype == CardSupertype::Champion)
    }

    /// Count the number of copies of the given card present in this deck.
    ///
    /// # Example
    ///
    /// ```rust
    /// use patched_porobot::deck;
    /// use patched_porobot::data::deckcode::deck::Deck;
    /// use patched_porobot::data::setbundle::code::CardCode;
    ///
    /// let d: Deck = deck![
    ///     "01DE002": 40,
    /// ];
    ///
    /// let copies = d.copies_of(&CardCode::from("01DE002".to_string()));
    /// assert_eq!(copies, 40);
    /// ```
    pub fn copies_of(&self, code: &CardCode) -> u32 {
        self.contents.get(code).map_or(0, ToOwned::to_owned)
    }

    /// Get the number of cards in the deck.
    ///
    /// In the *Standard* and *Singleton* formats, this is never more than 40.
    ///
    /// # Example
    ///
    /// ```rust
    /// use patched_porobot::deck;
    /// use patched_porobot::data::deckcode::deck::Deck;
    /// use patched_porobot::data::setbundle::card::CardIndex;
    /// use patched_porobot::data::setbundle::create_cardindex_from_wd;
    ///
    /// let index: CardIndex = create_cardindex_from_wd();
    /// let deck: Deck = deck!("CECQCAQCA4AQIAYKAIAQGLRWAQAQECAPEUXAIAQDAEBQOCIBAIAQEMJYAA");
    /// assert_eq!(deck.card_count(), 40);
    ///
    pub fn card_count(&self) -> u32 {
        self.contents.values().sum()
    }

    /// Get the number of champion cards in the deck.
    ///
    /// In the *Standard* and *Singleton* format, this is never more than 6.
    ///
    /// # Example
    ///
    /// ```rust
    /// use patched_porobot::deck;
    /// use patched_porobot::data::deckcode::deck::Deck;
    /// use patched_porobot::data::setbundle::card::CardIndex;
    /// use patched_porobot::data::setbundle::create_cardindex_from_wd;
    ///
    /// let index: CardIndex = create_cardindex_from_wd();
    /// let deck: Deck = deck!("CECQCAQCA4AQIAYKAIAQGLRWAQAQECAPEUXAIAQDAEBQOCIBAIAQEMJYAA");
    /// assert_eq!(deck.champions_count(&index), 6);
    /// ```
    pub fn champions_count(&self, cards: &CardIndex) -> u32 {
        self.champions(cards)
            .map(|c| &c.code)
            .map(|cc| self.copies_of(cc))
            .sum()
    }

    /// Find the first possible set of regions that the [`Deck`] fits in.
    ///
    /// Lower amounts of regions are preferred: the limit will be increased from 1 up to `limit` until a valid solution is found.
    ///
    /// # Warning
    ///
    /// This method traverses the tree of possible region selections until one is found.
    ///
    /// The time required to run this function may grow exponentially with the amount of cards in the deck!
    pub fn regions(&self, card_index: &CardIndex, limit: usize) -> Option<HashSet<CardRegion>> {
        let cards: Vec<&Card> = self.contents.keys()
            .flat_map(|cc| cc.to_card(card_index))
            .collect();

        for n in 1..=limit {
            let result = Self::regions_recursive_first_limit(cards.as_slice(), HashSet::new(), n);

            if result.is_some() {
                return result;
            }
        }

        None
    }

    /// Find the first possible set of regions that covers all given [`Card`]s.
    ///
    /// This function is *recursive*: the `cards` parameter holds the [`Card`]s left to process, while the `regions` parameter holds the regions found so far, and the `limit` parameter holds the size of the `regions` set to stop the recursion at.
    ///
    /// # Warning
    ///
    /// This method traverses the tree of possible region selections until one is found.
    ///
    /// The time required to run this function may grow exponentially with the size of `cards`!
    fn regions_recursive_first_limit(cards: &[&Card], regions: HashSet<CardRegion>, limit: usize) -> Option<HashSet<CardRegion>> {
        match cards.get(0) {
            None => Some(regions),
            Some(card) => {
                card.regions.iter()
                    .map(|region| {
                        match region {
                            CardRegion::Unsupported => {
                                None
                            }
                            CardRegion::Runeterra => {
                                Self::regions_recursive_first_limit(&cards[1..], regions.clone(), limit)
                            }
                            _ => {
                                let mut regions = regions.clone();
                                let inserted = regions.insert(*region);
                                match inserted && regions.len() > limit {
                                    true => None,
                                    false => Self::regions_recursive_first_limit(&cards[1..], regions, limit)
                                }
                            }
                        }
                    })
                    .find(Option::is_some)
                    .unwrap_or(None)
            },
        }
    }

    /// Check if the [`Deck`] is legal for play in the *Standard* format.
    ///
    /// # Returns
    ///
    /// - `None` if the deck is not legal for *Standard* play.
    /// - `Some(regions)` if the deck is legal for *Standard* play considering the specified region set.
    pub fn standard(&self, cards: &CardIndex) -> Option<HashSet<CardRegion>> {
        let copies_limit = self.contents.values().all(|n| n <= &3);
        let cards_limit = self.card_count() == 40;
        let champions_limit = self.champions_count(cards) <= 6;
        let regions = self.regions(cards, 2);

        match copies_limit && cards_limit && champions_limit {
            false => None,
            true => regions,
        }
    }

    /// Check if the [`Deck`] is legal for play in the *Singleton* format.
    pub fn singleton(&self, cards: &CardIndex) -> Option<HashSet<CardRegion>> {
        let copies_limit = self.contents.values().all(|n| n <= &1);
        let cards_limit = self.card_count() == 40;
        let champions_limit = self.champions_count(cards) <= 6;
        let regions = self.regions(cards, 3);

        match copies_limit && cards_limit && champions_limit {
            false => None,
            true => regions,
        }
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

/// The [`Result`] of a [`Deck`] **decoding** operation, for example [`Deck::from_code`].
pub type DeckDecodingResult<T> = Result<T, DeckDecodingError>;

/// The [`Result`] of a [`Deck`] **encoding** operation, for example [`Deck::to_code`].
pub type DeckEncodingResult<T> = Result<T, DeckEncodingError>;

/// Macro to build a deck.
///
/// Useful to quickly build [`Deck`]s from trusted input, such as when creating tests.
///
/// It can build a deck:
///
/// - from a deck code;
/// - from card code strings and quantities.
///
/// # Panics
///
/// If the deck code is not valid.
///
/// # Examples
///
/// ```rust
/// use patched_porobot::deck;
///
/// let _my_deck = deck!("CECQCAQCA4AQIAYKAIAQGLRWAQAQECAPEUXAIAQDAEBQOCIBAIAQEMJYAA");
/// ```
///
/// ```rust
/// use patched_porobot::deck;
///
/// let _my_deck = deck![
///     "01DE002": 3,
///     "01DE003": 3,
///     "01DE004": 3,
///     "01DE005": 3,
///     "01DE006": 3,
///     "01DE007": 3,
///     "01DE008": 3,
///     "01DE009": 3,
///     "01DE010": 3,
///     "01DE011": 3,
///     "01DE012": 3,
///     "01DE013": 3,
///     "01DE014": 3,
///     "01DE015": 3,
///     "01DE016": 3,
///     "01DE017": 3,
///     "01DE018": 3,
///     "01DE019": 3,
///     "01DE020": 3,
///     "01DE021": 3,
/// ];
/// ```
#[macro_export]
macro_rules! deck {
    [ $($cd:literal: $qty:expr),* $(,)? ] => {
        $crate::data::deckcode::deck::Deck {
            contents: std::collections::HashMap::from([
                $(($crate::data::setbundle::code::CardCode { full: $cd.to_string() }, $qty),)*
            ])
        }
    };

    ( $code:expr ) => {
        $crate::data::deckcode::deck::Deck::from_code($code).expect("deck code created with deck!() to be valid")
    };
}

#[rustfmt::skip::macros(test_de_ser, test_ser_de)]
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_de_ser {
        ( $id:ident, $src:literal ) => {
            #[test]
            fn $id() {
                let deck = Deck::from_code($src).expect("deck to deserialize successfully");
                let code = deck.to_code(DeckCodeFormat::F1).expect("deck to serialize successfully");
                assert_eq!(code, $src);
            }
        };
        ( $id:ident, $src:literal, $($tag:meta),* $(,)?) => {
            #[test]
            $(#[$tag])*
            fn $id() {
                let deck = Deck::from_code($src).expect("deck to deserialize successfully");
                let code = deck.to_code(DeckCodeFormat::F1).expect("deck to serialize successfully");
                assert_eq!(code, $src);
            }
        };
    }

    // riot's code is perfect and their examples always work correctly
    test_de_ser!(test_de_ser_riotexample,   "CEAAECABAQJRWHBIFU2DOOYIAEBAMCIMCINCILJZAICACBANE4VCYBABAILR2HRL", ignore);

    test_de_ser!(test_de_ser_yordlestar,    "CQBQCBAJBUCAKCRYHKTADNIBAYBQSDQ2DQ3FEWACAECQVNQBAIBQSOK5AEAQGCIV");
    test_de_ser!(test_de_ser_gimboclown,    "CICACAYGBAAQIBIBAMAQKKZPGEDQEBQEBEGBEFA2EYAQCAYGCABACAIFGUAQGBIH");
    test_de_ser!(test_de_ser_paltrisundisc, "CMCACAQGAMAQKBYOAEDAOMAEAQDSOPSCKMBAEBIHA4FQMBAHAEGA2HBMJQAQGBAHEVHWQ");
    test_de_ser!(test_de_ser_nomoretf,      "CMBQCBABCEBACAIXEYCQIBZWINQWO3IFAEBACBABAQAQUAIGAEOQEAIBBMVAEBAHHNCQEAIEA4TACBIHCY");
    test_de_ser!(test_de_ser_lonelyporo3,   "CEAQCAIBBAAAA");
    test_de_ser!(test_de_ser_lonelyporo2,   "CEAACAIBAEEAA");
    test_de_ser!(test_de_ser_lonelyporo1,   "CEAAAAIBAEAQQ");
    test_de_ser!(test_de_ser_someporos1,    "CQAAABIBAEAQQAIBAMRACAIECQAQGBATAECQVIAB");
    test_de_ser!(test_de_ser_steffonoxus,   "CECQCAQCA4AQIAYKAIAQGLRWAQAQECAPEUXAIAQDAEBQOCIBAIAQEMJYAA");
    test_de_ser!(test_de_ser_rampanivia,    "CECACBAAAUBQCAACA4VAGAYBBIJRMBIBAEGBQIJSGQAQCAIBGEBACAIAEAAQCAIU");
    test_de_ser!(test_de_ser_poromegamix,   "CQDACAQBAMAQIAIPAECQVIABAIAQIFAXAIBQIEQTAMAQCCAQGUCACAYBAIAQGBADAECQVDABAIAQCKZZAA");
    test_de_ser!(test_de_ser_hecapony,      "CMBAMAIFAQDRKJRKGEDQIBYDGNFWGZ3SPEAACAIEA45Q");
    test_de_ser!(test_de_ser_paltriunholy,  "CQCQCAYBAIAQIAIPAECQVIABAEDAOCIFAEAQGCAJCA2QIAICAEBQCBABBIAQMBYOAMCAOO2MNUAQCBQHBM");
    // test_de_ser!(test_de_ser_, "");

    macro_rules! test_ser_de {
        ( $id:ident, $deck:expr ) => {
            #[test]
            fn $id() {
                use crate::data::deckcode::deck::Deck;
                let deck1 = $deck;
                let code = deck1
                    .to_code(DeckCodeFormat::F1)
                    .expect("deck to serialize successfully");
                println!("Serialized deck code (for science, obviously): {}", &code);
                let deck2 = Deck::from_code(&code).expect("deck to deserialize successfully");
                assert_eq!(deck1, deck2);
            }
        };
    }

    // Some tests from https://github.com/RiotGames/LoRDeckCodes/blob/main/LoRDeckCodes_Tests/UnitTest1.cs

    test_ser_de!(test_ser_de_smalldeck, deck![
        "01DE002": 1,
    ]);
    test_ser_de!(test_ser_de_largedeck, deck![
        "01DE002": 3,
        "01DE003": 3,
        "01DE004": 3,
        "01DE005": 3,
        "01DE006": 3,
        "01DE007": 3,
        "01DE008": 3,
        "01DE009": 3,
        "01DE010": 3,
        "01DE011": 3,
        "01DE012": 3,
        "01DE013": 3,
        "01DE014": 3,
        "01DE015": 3,
        "01DE016": 3,
        "01DE017": 3,
        "01DE018": 3,
        "01DE019": 3,
        "01DE020": 3,
        "01DE021": 3,
    ]);
    test_ser_de!(test_ser_de_smallmorethan3, deck![
        "01DE002": 4,
    ]);
    test_ser_de!(test_ser_de_largemorethan3, deck![
        "01DE002": 3,
        "01DE003": 3,
        "01DE004": 3,
        "01DE005": 3,
        "01DE006": 4,
        "01DE007": 5,
        "01DE008": 6,
        "01DE009": 7,
        "01DE010": 8,
        "01DE011": 9,
        "01DE012": 3,
        "01DE013": 3,
        "01DE014": 3,
        "01DE015": 3,
        "01DE016": 3,
        "01DE017": 3,
        "01DE018": 3,
        "01DE019": 3,
        "01DE020": 3,
        "01DE021": 3,
    ]);
    test_ser_de!(test_ser_de_single40, deck![
        "01DE002": 40,
    ]);
    test_ser_de!(test_ser_de_worstcaselength, deck![
        "01DE002": 4,
        "01DE003": 4,
        "01DE004": 4,
        "01DE005": 4,
        "01DE006": 4,
        "01DE007": 5,
        "01DE008": 6,
        "01DE009": 7,
        "01DE010": 8,
        "01DE011": 9,
        "01DE012": 4,
        "01DE013": 4,
        "01DE014": 4,
        "01DE015": 4,
        "01DE016": 4,
        "01DE017": 4,
        "01DE018": 4,
        "01DE019": 4,
        "01DE020": 4,
        "01DE021": 4,
    ]);
    test_ser_de!(test_ser_de_bilgewater, deck![
        "01DE002": 4,
        "02BW003": 2,
        "02BW010": 3,
        "01DE004": 5,
    ]);
    test_ser_de!(test_ser_de_shurima, deck![
        "01DE002": 4,
        "02BW003": 2,
        "02BW010": 3,
        "04SH047": 5,
    ]);
    test_ser_de!(test_ser_de_targon, deck![
        "01DE002": 4,
        "03MT003": 2,
        "03MT010": 3,
        "02BW004": 5,
    ]);
    test_ser_de!(test_ser_de_runeterra, deck![
        "01DE002": 4,
        "03MT003": 2,
        "03MT010": 3,
        "01RU001": 5,
    ]);
    test_ser_de!(test_ser_de_morepowder, deck![
        "02BW012": 69,
    ]);

    // test_ser_de!(test_ser_de_, deck![]);

    macro_rules! test_legality {
        ( $id:ident, $deck:expr, $check:path, $assert:expr ) => {
            #[test]
            fn $id() {
                let index = $crate::data::setbundle::create_cardindex_from_wd();
                let deck: Deck = $deck;
                let result = $check(&deck, &index).is_some();
                assert_eq!(result, $assert);
            }
        }
    }

    test_legality!(
        test_legality_standard_lonelyporo1,
        deck!("CEAAAAIBAEAQQ"),
        Deck::standard, false
    );
    test_legality!(
        test_legality_standard_twistedshrimp,
        deck!("CICACBAFAEBAGBQICABQCBJLF4YQOAQGAQEQYEQUDITAAAIBAMCQO"),
        Deck::standard, true
    );
    test_legality!(
        test_legality_standard_poros,
        deck!("CQDQCAQBAMAQGAICAECACDYCAECBIFYCAMCBEEYCAUFIYANAAEBQCAIICA2QCAQBAEVTSAA"),
        Deck::standard, true
    );
    test_legality!(
        test_legality_standard_sand,
        deck!("CMBAGBAHANTXEBQBAUCAOFJGFIYQEAIBAUOQIBAHGM5HM6ICAECAOOYCAECRSGY"),
        Deck::standard, true
    );

    test_legality!(
        test_legality_singleton_lonelyporo1,
        deck!("CEAAAAIBAEAQQ"),
        Deck::singleton, false
    );
    test_legality!(
        test_legality_singleton_twistedshrimp,
        deck!("CICACBAFAEBAGBQICABQCBJLF4YQOAQGAQEQYEQUDITAAAIBAMCQO"),
        Deck::singleton, false
    );
    test_legality!(
        test_legality_singleton_poros,
        deck!("CQDQCAQBAMAQGAICAECACDYCAECBIFYCAMCBEEYCAUFIYANAAEBQCAIICA2QCAQBAEVTSAA"),
        Deck::singleton, false
    );
    test_legality!(
        test_legality_singleton_sand,
        deck!("CMBAGBAHANTXEBQBAUCAOFJGFIYQEAIBAUOQIBAHGM5HM6ICAECAOOYCAECRSGY"),
        Deck::singleton, false
    );
    test_legality!(
        test_legality_singleton_paltri,
        deck!("CQAAADABAICACAIFBLAACAIFAEHQCBQBEQBAGBADAQBAIAIKBUBAKBAWDUBQIBACA4GAMAIBAMCAYHJBGADAMBAOCQKRMKBLA4AQIAQ3D4QSIKZYBACAODJ3JRIW3AABQIAYUAI"),
        Deck::singleton, true
    );
}
