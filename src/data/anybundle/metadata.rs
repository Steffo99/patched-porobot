//! Module defining [BundleMetadata], the contents of the `metadata.json` present in all [Data Dragon] Bundles.
//!
//! [Data Dragon]: https://developer.riotgames.com/docs/lor#data-dragon

use std::fs::File;
use std::path::Path;
use crate::data::anybundle::outcomes::{LoadingError, LoadingResult};

/// A parsed `metadata.json` file from a Data Dragon Bundle.
///
/// The specification defines more fields, but they are missing from the output files:
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
    /// Load a `metadata.json` file to create a [BundleMetadata] instance.
    pub fn load(path: &Path) -> LoadingResult<Self> {
        let file = File::open(path)
            .map_err(LoadingError::OpeningFile)?;
        let data = serde_json::de::from_reader::<File, Self>(file)
            .map_err(LoadingError::Deserializing)?;
        Ok(data)
    }

    /// Get a reference to the first (and probably only) locale defined in BundleMetadata.
    ///
    /// Equivalent to calling [BundleMetadata].[locales](BundleMetadata::locales).[get(0)]([T]::get).
    pub fn locale(&self) -> Option<&String> {
        self.locales.get(0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, BundleMetadata>(r#"
                {
                    "locales": [
                        "en_us"
                    ]
                }
            "#).unwrap(),
            BundleMetadata {
                locales: vec!["en_us".to_string()]
            }
        );
    }
}
