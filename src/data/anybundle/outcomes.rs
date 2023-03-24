//! Module defining [Result] and [Result::Err] variants for actions related to [Data Dragon] Bundles.
//!
//! [Data Dragon]: https://developer.riotgames.com/docs/lor#data-dragon

/// An error that occoured while loading a Data Dragon Bundle.
#[derive(Debug)]
pub enum LoadingError {
    /// Could not get the locale from the `metadata.json` file.
    GettingLocale,
    /// Could not get the bundle name from the operating system.
    GettingBundleName,
    /// Could not use [File::open](std::fs::File::open) on a data file.
    OpeningFile(std::io::Error),
    /// Could not deserialize a data file.
    Deserializing(serde_json::Error),
    /// Could not fetch a data file from a remote location.
    RemoteFetching(reqwest::Error),
    /// Could not deserialize a data file from a remote location.
    RemoteDeserializing(reqwest::Error),
}

/// The result of the loading of a Legends of Runeterra bundle.
pub type LoadingResult<T> = Result<T, LoadingError>;
