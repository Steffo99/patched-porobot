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
    /// The contents of the `[locale]/data/globals-[locale].json` file.
    pub globals: globals::LocalizedGlobalsVecs,
}

impl CoreBundle {
    /// Load a Core Bundle directory to create a [CoreBundle] instance.
    pub fn load(bundle_path: &Path) -> LoadingResult<Self> {
        let metadata = BundleMetadata::load(&bundle_path.join("metadata.json"))?;

        let locale = metadata.locale().ok_or(LoadingError::GettingLocale)?;

        let globals_path = &bundle_path
            .join(locale)
            .join("data")
            .join(format!("globals-{}.json", &locale));

        let globals = globals::LocalizedGlobalsVecs::load(globals_path)?;

        Ok(CoreBundle {
            globals,
        })
    }

    /// Fetch from `base_url` the Core Bundle data with the given `locale`.
    #[cfg(feature = "fetch")]
    pub async fn fetch(client: &reqwest::Client, base_url: &str, locale: &str) -> LoadingResult<Self> {
        let globals = client
            .get(format!("{base_url}/core/{locale}/data/globals-{locale}.json"))
            .send()
            .await
            .map_err(LoadingError::RemoteFetching)?
            .json::<globals::LocalizedGlobalsVecs>()
            .await
            .map_err(LoadingError::RemoteDeserializing)?;

        Ok(Self {globals})
    }
}


/// Create [`globals::LocalizedGlobalsIndexes`] from the core bundle in the current working directory.
///
/// This function tries to load data from the first directory matching the [glob] `./data/core-*`.
pub fn create_globalindexes_from_wd() -> globals::LocalizedGlobalsIndexes {
    let path = glob::glob("./data/core-*")
        .expect("glob to be a valid glob")
        .filter_map(Some)
        .find_map(Result::ok)
        .expect("a valid core bundle to exist");

    let core = CoreBundle::load(&path)
        .expect("to be able to load `core-en_us` bundle");

    globals::LocalizedGlobalsIndexes::from(core.globals)
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_fetch {
        ( $id:ident, $version:literal, $locale:literal ) => {
            #[cfg(feature = "fetch")]
            #[tokio::test]
            async fn $id() {
                let client = reqwest::Client::new();
                let result = CoreBundle::fetch(&client, &format!("https://dd.b.pvp.net/{}", $version), $locale).await;
                assert!(result.is_ok());
            }
        };
    }

    test_fetch!(test_fetch_3_17_0_en_us, "3_17_0", "en_us");
    test_fetch!(test_fetch_3_17_0_it_it, "3_17_0", "it_it");
    test_fetch!(test_fetch_latest_en_us, "latest", "en_us");
}