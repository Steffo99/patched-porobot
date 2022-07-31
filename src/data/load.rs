use std::path::*;
use glob::{glob, GlobResult};
use itertools::Itertools;
use crate::data::schema::Card;
use log::*;


#[derive(Debug)]
enum LoadingError {
    IO(std::io::Error),
    Parsing(serde_json::Error),
}


/// Load a single data file.
fn load_file(path: PathBuf) -> Result<Vec<Card>, LoadingError> {
    let file = std::fs::File::open(path)
        .map_err(LoadingError::IO)?;
    let data = serde_json::de::from_reader::<std::fs::File, Vec<Card>>(file)
        .map_err(LoadingError::Parsing)?;
    Ok(data)
}


/// Load a single data file, and handle errors by logging them as warnings.
fn load_file_and_warn(path: PathBuf) -> Vec<Card> {
    match load_file(path) {
        Ok(v) => v,
        Err(e) => {
            warn!("{:?}", e);
            vec![]
        }
    }
}


/// Create a [Vec] of all cards contained in the data files.
pub fn load_files() -> Vec<Card> {
    glob("./data/*/en_us/data/*.json")
        .unwrap()
        .filter_map(GlobResult::ok)
        .map(load_file_and_warn)
        .concat()
}
