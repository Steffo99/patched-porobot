//! Module defining the [`DeckCodeVersion`] enum and [`DeckCodeVersioned`] trait.

use std::cmp::max;
use crate::data::setbundle::code::CardCode;
use crate::data::setbundle::region::CardRegion;
use crate::data::setbundle::set::CardSet;


/// The version of a deck code.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeckCodeVersion {
    /// > Closed alpha. Supports original set.
    V1 = 1,

    /// > Launch. Supports second set with the Bilgewater faction.
    ///
    /// > Supports third set with the Targon faction.
    V2 = 2,

    /// > Supports Empires of the Ascended expansion with Shurima faction.
    V3 = 3,

    /// > Supports Beyond the Bandlewood expansion with Bandle City faction and an update to the deck code library which will create the lowest version code required based on the cards in the deck.
    V4 = 4,

    /// > Supports Worldwalker expansion with Runeterra faction.
    V5 = 5,
}


/// Indicates that a given type is versioned in the [specs for deck codes](https://github.com/RiotGames/LoRDeckCodes).
pub trait DeckCodeVersioned {
    /// Get the minimum deck version required to encode the deck in "code" format.
    fn min_deck_version(&self) -> Option<DeckCodeVersion>;
}


impl DeckCodeVersioned for CardSet {
    fn min_deck_version(&self) -> Option<DeckCodeVersion> {
        match self {
            CardSet::Foundations => Some(DeckCodeVersion::V1),

            CardSet::RisingTides => Some(DeckCodeVersion::V2),
            CardSet::CallOfTheMountain => Some(DeckCodeVersion::V2),

            CardSet::EmpiresOfTheAscended => Some(DeckCodeVersion::V3),

            CardSet::BeyondTheBandlewood => Some(DeckCodeVersion::V4),

            CardSet::Worldwalker => Some(DeckCodeVersion::V5),

            _ => None,
        }
    }
}


impl DeckCodeVersioned for CardRegion {
    fn min_deck_version(&self) -> Option<DeckCodeVersion> {
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


/// [`CardCode`]s versions are the maximum version of their components.
impl DeckCodeVersioned for CardCode {
    fn min_deck_version(&self) -> Option<DeckCodeVersion> {
        let set = CardSet::from_code(self.set()).min_deck_version();
        if set.is_none() { return None };

        let region = CardRegion::from_code(self.region()).min_deck_version();
        if region.is_none() { return None };

        max(set, region)
    }
}
