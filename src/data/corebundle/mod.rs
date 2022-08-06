//! This module defines the types used in Data Dragon [Core Bundles](https://developer.riotgames.com/docs/lor#data-dragon_core-bundles).

use std::path::Path;
use super::anybundle::metadata::BundleMetadata;
use super::outcomes::{LoadingError, LoadingResult};

pub mod globals;
pub mod vocabterm;
pub mod keyword;
pub mod region;
pub mod speed;
pub mod rarity;
pub mod set;


/// A parsed [Core Bundle](https://developer.riotgames.com/docs/lor#data-dragon_core-bundles).
pub struct CoreBundle {
    /// The contents of the `metadata.json` file.
    pub metadata: BundleMetadata,

    /// The contents of the `[locale]/data/globals-[locale].json` file.
    pub globals: globals::LocalizedGlobalsVecs,
}


impl CoreBundle {
    /// Load a Core Bundle directory to create a [CoreBundle] instance.
    pub fn load(bundle_path: &Path) -> LoadingResult<Self> {
        let metadata = BundleMetadata::load(
            &bundle_path
                .join("metadata.json")
        )?;

        let locale = metadata.locale().ok_or(LoadingError::Using)?;

        let globals = globals::LocalizedGlobalsVecs::load(
            &bundle_path
                .join(&locale)
                .join("data")
                .join(format!("globals-{}.json", &locale))
        )?;

        Ok(CoreBundle {metadata, globals})
    }
}
