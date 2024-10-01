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
pub mod format;

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
        log::debug!("Loading CoreBundle from: {}", bundle_path.as_os_str().to_string_lossy());

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
    pub async fn fetch(client: &reqwest::Client, base_url: &str, locale: &str) -> LoadingResult<Self> {
        let url = format!("{}/core/{}/data/globals-{}.json", base_url, locale, locale);

        log::debug!("Fetching CoreBundle from {} ...", &url);

        let globals = client
            .get(url)
            .send()
            .await
            .map_err(LoadingError::RemoteFetching)?
            .json::<globals::LocalizedGlobalsVecs>()
            .await
            .map_err(LoadingError::RemoteDeserializing)?;

        log::debug!("Fetched CoreBundle: it defines {} regions, {} keywords, {} rarities, {} sets, {} spell speeds, and {} vocab terms!", &globals.regions.len(), &globals.keywords.len(), &globals.rarities.len(), &globals.sets.len(), &globals.spell_speeds.len(), &globals.vocab_terms.len());

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
        .expect("to be able to load CoreBundle bundle");

    globals::LocalizedGlobalsIndexes::from(core.globals)
}


/// Create [`globals::LocalizedGlobalsIndexes`] from the latest data in Data Dragon.
///
/// This function tries to load data from `https://dd.b.pvp.net/latest`.
pub async fn create_globalindexes_from_dd_latest(locale: &str) -> globals::LocalizedGlobalsIndexes {
    let client = reqwest::Client::new();

    let core = CoreBundle::fetch(&client, "https://dd.b.pvp.net/latest", locale).await
        .expect("to be able to fetch CoreBundle");

    globals::LocalizedGlobalsIndexes::from(core.globals)
}


#[cfg(test)]
mod tests {
    use crate::data::setbundle::format::CardFormat;
    use crate::data::setbundle::keyword::CardKeyword;
    use crate::data::setbundle::rarity::CardRarity;
    use crate::data::setbundle::region::CardRegion;
    use crate::data::setbundle::set::CardSet;
    use crate::data::setbundle::speed::SpellSpeed;
    macro_rules! test_fetch {
        ( $id:ident, $version:literal, $locale:literal ) => {
            #[tokio::test]
            async fn $id() {
                let client = reqwest::Client::new();
                let result = crate::data::corebundle::CoreBundle::fetch(&client, &format!("https://dd.b.pvp.net/{}", $version), $locale).await;
                assert!(result.is_ok());
            }
        };
    }

    test_fetch!(test_fetch_5_9_0_en_us, "5_9_0", "en_us");
    test_fetch!(test_fetch_5_9_0_it_it, "5_9_0", "it_it");
    test_fetch!(test_fetch_latest_en_us, "latest", "en_us");

    macro_rules! test_supported {
        ( $id:ident, $version:literal, $locale:literal ) => {
            #[tokio::test]
            async fn $id() {
                let client = reqwest::Client::new();
                let result = crate::data::corebundle::CoreBundle::fetch(&client, &format!("https://dd.b.pvp.net/{}", $version), $locale).await;
                let result = result.expect("fetch request to be successful");

                result.globals.keywords.iter().for_each(|o| assert_ne!(o.keyword, CardKeyword::Unsupported, "{:?} is unsupported", o));
                result.globals.regions.iter().for_each(|o| assert_ne!(o.region, CardRegion::Unsupported, "{:?} is unsupported", o));
                result.globals.spell_speeds.iter().for_each(|o| assert_ne!(o.spell_speed, SpellSpeed::Unsupported, "{:?} is unsupported", o));
                result.globals.rarities.iter().for_each(|o| assert_ne!(o.rarity, CardRarity::Unsupported, "{:?} is unsupported", o));
                result.globals.sets.iter().for_each(|o| assert_ne!(o.set, CardSet::Unsupported, "{:?} is unsupported", o));
                result.globals.formats.iter().for_each(|o| assert_ne!(o.format, CardFormat::Unsupported, "{:?} is unsupported", o));
            }
        };
    }

    test_supported!(test_supported_5_9_0_en_us, "5_9_0", "en_us");
    test_supported!(test_supported_5_9_0_it_it, "5_9_0", "it_it");
    test_supported!(test_supported_latest_en_us, "latest", "en_us");
}
