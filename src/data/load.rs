//! This module contains functions to load **Set Bundles** from [Data Dragon](https://developer.riotgames.com/docs/lol).

use itertools::Itertools;
use crate::data::schema::Card;


#[derive(Debug)]
enum LoadingError {
    IO(std::io::Error),
    Parsing(serde_json::Error),
}


/// Load a single Set Bundle and create a [Vec] with the cards contained in it.
fn load_setbundle(path: std::path::PathBuf) -> Result<Vec<Card>, LoadingError> {
    let file = std::fs::File::open(path)
        .map_err(LoadingError::IO)?;
    let data = serde_json::de::from_reader::<std::fs::File, Vec<Card>>(file)
        .map_err(LoadingError::Parsing)?;
    Ok(data)
}


/// Load a single Set Bundle (similarly to [load_setbundle]), but instead of returning a [Result], return an empty [Vec] in case of failure and log a [warn]ing.
fn load_setbundle_infallible(path: std::path::PathBuf) -> Vec<Card> {
    match load_setbundle(path) {
        Ok(v) => v,
        Err(e) => {
            log::warn!("{:?}", e);
            vec![]
        }
    }
}


/// Load all Set Bundles matched by the passed glob, using [load_setbundle_infallible] and then concatenating the resulting [Vec]s.
pub fn load_setbundles_infallible(pattern: &str) -> Vec<Card> {
    glob::glob(pattern)
        .expect("a valid glob")
        .filter_map(glob::GlobResult::ok)
        .map(load_setbundle_infallible)
        .concat()
}
