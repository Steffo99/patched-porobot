//! Module defining the types used in [Data Dragon] [Set Bundle]s.
//!
//! [Data Dragon]: https://developer.riotgames.com/docs/lor#data-dragon
//! [Set Bundle]: https://developer.riotgames.com/docs/lor#data-dragon_set-bundles

use super::anybundle::metadata::BundleMetadata;
use crate::data::anybundle::outcomes::{LoadingError, LoadingResult};
use std::fs::File;
use std::path::{Path, PathBuf};

pub mod art;
pub mod card;
pub mod code;
pub mod keyword;
pub mod rarity;
pub mod region;
pub mod set;
pub mod speed;
pub mod subtype;
pub mod supertype;
pub mod r#type;

/// A parsed [Data Dragon] [Set Bundle].
///
/// [Data Dragon]: https://developer.riotgames.com/docs/lor#data-dragon
/// [Set Bundle]: https://developer.riotgames.com/docs/lor#data-dragon_set-bundles
pub struct SetBundle {
    /// The contents of the `metadata.json` file.
    pub metadata: BundleMetadata,

    /// The contents of the `[locale]/data/globals-[locale].json` file.
    pub cards: Vec<card::Card>,

    /// The name of the root directory of the bundle.
    pub name: String,
}

impl SetBundle {
    /// Load a Set Bundle directory to create a [SetBundle] instance.
    pub fn load(bundle_path: &Path) -> LoadingResult<Self> {
        let metadata = BundleMetadata::load(&bundle_path.join("metadata.json"))?;

        let locale = metadata.locale().ok_or(LoadingError::GettingLocale)?;

        let name = bundle_path
            .file_name()
            .ok_or(LoadingError::GettingBundleName)?;

        let data_path = {
            let mut json_filename = name.to_os_string();
            json_filename.push(".json");

            &bundle_path.join(locale).join("data").join(&json_filename)
        };

        let name = name
            .to_str()
            .ok_or(LoadingError::ConvertingBundleName)?
            .to_string();

        let cards = File::open(data_path).map_err(LoadingError::OpeningFile)?;

        let cards = serde_json::de::from_reader::<File, Vec<card::Card>>(cards)
            .map_err(LoadingError::Deserializing)?;

        Ok(SetBundle {
            metadata,
            cards,
            name,
        })
    }
}


/// Create a [`card::CardIndex`] from set bundles in the given paths.
///
/// # Panics
///
/// If any of the required files cannot be loaded (see [`SetBundle::load`]).
pub fn create_cardindex_from_paths(paths: impl Iterator<Item = PathBuf>) -> card::CardIndex {
    let mut index = card::CardIndex::new();
    for path in paths {
        let set = SetBundle::load(&path).expect("to be able to load SetBundle");
        for card in set.cards {
            index.insert(card.code.clone(), card);
        }
    };
    index
}

/// Create a [`card::CardIndex`] from set bundles in the current working directory.
///
/// This function tries to load data from any directory matching the [glob] `./data/set*-*`.
///
/// # Panics
///
/// See [`create_cardindex_from_paths`].
pub fn create_cardindex_from_wd() -> card::CardIndex {
    let paths = glob::glob("./data/set*-*")
        .expect("glob to be a valid glob")
        .filter_map(Some)
        .filter_map(Result::ok);

    create_cardindex_from_paths(paths)
}

