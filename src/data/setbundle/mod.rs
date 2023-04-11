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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetBundle {
    /// The contents of the `[locale]/data/globals-[locale].json` file.
    pub cards: Vec<card::Card>,
}

impl SetBundle {
    /// Load a Set Bundle directory to create a [SetBundle] instance.
    pub fn load(bundle_path: &Path) -> LoadingResult<Self> {
        log::debug!("Loading SetBundle from: {}", bundle_path.as_os_str().to_string_lossy());

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

        let cards = File::open(data_path).map_err(LoadingError::OpeningFile)?;

        let cards = serde_json::de::from_reader::<File, Vec<card::Card>>(cards)
            .map_err(LoadingError::Deserializing)?;

        Ok(SetBundle {
            cards,
        })
    }

    /// Fetch from `base_url` the Set Bundle data of the given `set` with the given `locale`.
    pub async fn fetch(client: &reqwest::Client, base_url: &str, locale: &str, set: &str) -> LoadingResult<Self> {
        let url = format!("{}/{}/{}/data/{}-{}.json", base_url, set, locale, set, locale);

        log::debug!("Fetching SetBundle from {} ...", url);

        let cards = client
            .get(url)
            .send()
            .await
            .map_err(LoadingError::RemoteFetching)?
            .json::<Vec<card::Card>>()
            .await
            .map_err(LoadingError::RemoteDeserializing)?;

        log::debug!("Fetched SetBundle: it defines {} cards!", cards.len());

        Ok(Self {cards})
    }
}


#[cfg(test)]
mod tests {
    macro_rules! test_fetch {
        ( $id:ident, $version:literal, $locale:literal, $set:literal ) => {
            #[tokio::test]
            async fn $id() {
                let client = reqwest::Client::new();
                let result = crate::data::setbundle::SetBundle::fetch(&client, &format!("https://dd.b.pvp.net/{}", $version), $locale, $set).await;
                assert!(result.is_ok());
            }
        };
    }

    test_fetch!(test_fetch_3_17_0_en_us_set1, "3_17_0", "en_us", "set1");
    test_fetch!(test_fetch_3_17_0_en_us_set2, "3_17_0", "en_us", "set2");
    test_fetch!(test_fetch_3_17_0_en_us_set3, "3_17_0", "en_us", "set3");
    test_fetch!(test_fetch_3_17_0_en_us_set4, "3_17_0", "en_us", "set4");
    test_fetch!(test_fetch_3_17_0_en_us_set5, "3_17_0", "en_us", "set5");
    test_fetch!(test_fetch_3_17_0_en_us_set6, "3_17_0", "en_us", "set6");
    test_fetch!(test_fetch_3_17_0_en_us_set6cde, "3_17_0", "en_us", "set6cde");
    
    test_fetch!(test_fetch_3_17_0_it_it_set1, "3_17_0", "it_it", "set1");
    test_fetch!(test_fetch_3_17_0_it_it_set2, "3_17_0", "it_it", "set2");
    test_fetch!(test_fetch_3_17_0_it_it_set3, "3_17_0", "it_it", "set3");
    test_fetch!(test_fetch_3_17_0_it_it_set4, "3_17_0", "it_it", "set4");
    test_fetch!(test_fetch_3_17_0_it_it_set5, "3_17_0", "it_it", "set5");
    test_fetch!(test_fetch_3_17_0_it_it_set6, "3_17_0", "it_it", "set6");
    test_fetch!(test_fetch_3_17_0_it_it_set6cde, "3_17_0", "it_it", "set6cde");

    test_fetch!(test_fetch_latest_en_us_set1, "latest", "en_us", "set1");
    test_fetch!(test_fetch_latest_en_us_set2, "latest", "en_us", "set2");
    test_fetch!(test_fetch_latest_en_us_set3, "latest", "en_us", "set3");
    test_fetch!(test_fetch_latest_en_us_set4, "latest", "en_us", "set4");
    test_fetch!(test_fetch_latest_en_us_set5, "latest", "en_us", "set5");
    test_fetch!(test_fetch_latest_en_us_set6, "latest", "en_us", "set6");
    test_fetch!(test_fetch_latest_en_us_set6cde, "latest", "en_us", "set6cde");
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

/// Create a [`card::CardIndex`] from the latest known data in Data Dragon.
///
/// This function tries to load data from `https://dd.b.pvp.net/latest`.
pub async fn create_cardindex_from_dd_latest(locale: &str, known_set_codes: impl Iterator<Item = &str>) -> card::CardIndex {
    let client = reqwest::Client::new();
    let mut index = card::CardIndex::new();

    for set_code in known_set_codes {
        let set = SetBundle::fetch(&client, "https://dd.b.pvp.net/latest", locale, set_code).await
            .expect("to be able to fetch set bundle");

        for card in set.cards {
            index.insert(card.code.clone(), card);
        }
    };

    index
}