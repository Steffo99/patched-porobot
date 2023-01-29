//! Module defining the types used in [Data Dragon] [Core Bundle]s.
//!
//! [Data Dragon]: https://developer.riotgames.com/docs/lor#data-dragon
//! [Core Bundle]: https://developer.riotgames.com/docs/lor#data-dragon_core-bundles

use super::anybundle::metadata::BundleMetadata;
use crate::data::anybundle::outcomes::{LoadingError, LoadingResult};
use std::path::Path;

pub mod globals;
pub mod keyword;
pub mod rarity;
pub mod region;
pub mod set;
pub mod speed;
pub mod vocabterm;

/// A parsed [Data Dragon] [Core Bundle].
///
/// [Data Dragon]: https://developer.riotgames.com/docs/lor#data-dragon
/// [Core Bundle]: https://developer.riotgames.com/docs/lor#data-dragon_core-bundles
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CoreBundle {
    /// The name of the root directory of the bundle.
    pub name: String,

    /// The contents of the `metadata.json` file.
    pub metadata: BundleMetadata,

    /// The contents of the `[locale]/data/globals-[locale].json` file.
    pub globals: globals::LocalizedGlobalsVecs,
}

impl CoreBundle {
    /// Load a Core Bundle directory to create a [CoreBundle] instance.
    pub fn load(bundle_path: &Path) -> LoadingResult<Self> {
        let metadata = BundleMetadata::load(&bundle_path.join("metadata.json"))?;

        let name = bundle_path
            .file_name()
            .ok_or(LoadingError::GettingBundleName)?
            .to_str()
            .ok_or(LoadingError::ConvertingBundleName)?
            .to_string();

        let locale = metadata.locale().ok_or(LoadingError::GettingLocale)?;

        let globals_path = &bundle_path
            .join(locale)
            .join("data")
            .join(format!("globals-{}.json", &locale));

        let globals = globals::LocalizedGlobalsVecs::load(globals_path)?;

        Ok(CoreBundle {
            name,
            metadata,
            globals,
        })
    }
}
