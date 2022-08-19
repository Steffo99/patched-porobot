//! Module defining the [`DeckCodeVersion`] enum and [`DeckCodeVersioned`] trait.

use std::cmp::max;
use crate::data::deckcode::deck::Deck;
use crate::data::setbundle::code::CardCode;
use crate::data::setbundle::region::CardRegion;
use crate::data::setbundle::set::CardSet;


/// The version of a deck code.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeckCodeVersion {
    /// > Closed alpha. Supports original set.
    V1,

    /// > Launch. Supports second set with the Bilgewater faction.
    ///
    /// > Supports third set with the Targon faction.
    V2,

    /// > Supports Empires of the Ascended expansion with Shurima faction.
    V3,

    /// > Supports Beyond the Bandlewood expansion with Bandle City faction and an update to the deck code library which will create the lowest version code required based on the cards in the deck.
    V4,

    /// > Supports Worldwalker expansion with Runeterra faction.
    V5,
}


impl TryFrom<u8> for DeckCodeVersion {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::V1),
            2 => Ok(Self::V2),
            3 => Ok(Self::V3),
            4 => Ok(Self::V4),
            5 => Ok(Self::V5),
            _ => Err(())
        }
    }
}

impl From<DeckCodeVersion> for u8 {
    fn from(value: DeckCodeVersion) -> Self {
        match value {
            DeckCodeVersion::V1 => 1,
            DeckCodeVersion::V2 => 2,
            DeckCodeVersion::V3 => 3,
            DeckCodeVersion::V4 => 4,
            DeckCodeVersion::V5 => 5,
        }
    }
}



/// Indicates that a given type is versioned in the [specs for deck codes](https://github.com/RiotGames/LoRDeckCodes).
pub trait DeckCodeVersioned {
    /// Get the minimum deck version required to encode the deck in "code" format.
    fn min_deckcode_version(&self) -> Option<DeckCodeVersion>;
}


impl DeckCodeVersioned for CardRegion {
    fn min_deckcode_version(&self) -> Option<DeckCodeVersion> {
        match self {
            CardRegion::Noxus => Some(DeckCodeVersion::V1),
            CardRegion::Demacia => Some(DeckCodeVersion::V1),
            CardRegion::Freljord => Some(DeckCodeVersion::V1),
            CardRegion::ShadowIsles => Some(DeckCodeVersion::V1),
            CardRegion::PiltoverZaun => Some(DeckCodeVersion::V1),
            CardRegion::Ionia => Some(DeckCodeVersion::V1),

            CardRegion::Targon => Some(DeckCodeVersion::V2),
            CardRegion::Bilgewater => Some(DeckCodeVersion::V2),

            CardRegion::Shurima => Some(DeckCodeVersion::V3),

            CardRegion::BandleCity => Some(DeckCodeVersion::V4),

            CardRegion::Runeterra => Some(DeckCodeVersion::V5),
            CardRegion::Jhin => Some(DeckCodeVersion::V5),
            CardRegion::Evelynn => Some(DeckCodeVersion::V5),
            CardRegion::Bard => Some(DeckCodeVersion::V5),

            _ => None,
        }
    }
}


/// [`CardCode`]'s version is the maximum version of its components.
impl DeckCodeVersioned for CardCode {
    fn min_deckcode_version(&self) -> Option<DeckCodeVersion> {
        CardRegion::from_code(self.region()).min_deckcode_version()
    }
}


/// [`Deck`]'s version is the maximum version of all its [`CardCode`]s.
impl DeckCodeVersioned for Deck {
    fn min_deckcode_version(&self) -> Option<DeckCodeVersion> {
        let codes = self.contents.keys();
        let versions = codes.map(|cc| cc.min_deckcode_version());

        // TODO: I'm almost sure this can be optimized, but it's 5 AM
        for version in versions.clone() {
            if version.is_none() {
                return None;
            }
        }

        let versions = versions.map(|v| v.unwrap());

        versions.max()
    }
}
