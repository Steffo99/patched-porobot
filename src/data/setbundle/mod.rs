//! This module defines the types used in Data Dragon [Set Bundles](https://developer.riotgames.com/docs/lor#data-dragon_set-bundles).

use std::fs::File;
use std::path::Path;
use super::anybundle::metadata::BundleMetadata;
use super::outcomes::{LoadingError, LoadingResult};

pub mod card;
pub mod art;
pub mod r#type;
pub mod rarity;
pub mod region;
pub mod set;
pub mod speed;
pub mod keyword;


/// A parsed [Set Bundle](https://developer.riotgames.com/docs/lor#data-dragon_set-bundles).
pub struct SetBundle {
    /// The contents of the `metadata.json` file.
    pub metadata: BundleMetadata,

    /// The contents of the `[locale]/data/globals-[locale].json` file.
    pub cards: Vec<card::Card>,
}


impl SetBundle {
    /// Load a Set Bundle directory to create a [SetBundle] instance.
    pub fn load(bundle_path: &Path) -> LoadingResult<Self> {
        let metadata = BundleMetadata::load(
            &bundle_path
                .join("metadata.json")
        )?;

        let locale = metadata.locale().ok_or(LoadingError::Using)?;

        let mut filename = bundle_path.file_name().ok_or(LoadingError::Checking)?.to_os_string();
        filename.push(".json");

        let cards = File::open(
            &bundle_path
                .join(&locale)
                .join("data")
                .join(filename)
        ).map_err(LoadingError::Loading)?;

        let cards = serde_json::de::from_reader::<File, Vec<card::Card>>(cards)
            .map_err(LoadingError::Parsing)?;

        Ok(SetBundle {metadata, cards})
    }
}
