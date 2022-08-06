//! This module defines [BundleMetadata], the contents of `metadata.json`.

use std::fs::File;
use std::path::Path;
use crate::data::outcomes::{LoadingError, LoadingResult};

/// A parsed `metadata.json` file from a Data Dragon Bundle.
///
/// The specification defines more fields, but they are missing from the output files.
///
/// > ```json
/// > {
/// >     "locales": ["{string}", ],
/// >     "clientHash": "{string}"
/// >     "gameplayDataHash": "{string}",
/// >     "timestamp": "{YYYYMMDDhhmm}",
/// >     "patchlineRef": "{string}"
/// > }
/// > ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct BundleMetadata {
    /// [Vec] of locales included in the bundle.
    ///
    /// The specification defines that there can be multiple, but currently I've never seen more (or less) than one.
    pub locales: Vec<String>
}


impl BundleMetadata {
    /// Load a `metadata.json` file to create a [LocalizedGlobalsVecs] instance.
    pub fn load(path: &Path) -> LoadingResult<Self> {
        let file = File::open(path)
            .map_err(LoadingError::Loading)?;
        let data = serde_json::de::from_reader::<File, Self>(file)
            .map_err(LoadingError::Parsing)?;
        Ok(data)
    }

    /// Get a reference to the first (and probably only) locale defined in BundleMetadata.
    pub fn locale(&self) -> Option<&String> {
        self.locales.get(0)
    }
}