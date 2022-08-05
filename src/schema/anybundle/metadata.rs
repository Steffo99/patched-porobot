/// The contents of the `anybundle.json` file of a bundle.
///
/// Both Core Bundles and Set Bundles are bundles, and both have anybundle.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct BundleMetadata {
    /// The locales included in the bundle.
    ///
    /// I've never seen more that a single locale here, but the specification allows that.
    locales: Vec<String>,

    /// ???
    #[serde(default = "none")]
    client_hash: Option<String>,

    /// ???
    #[serde(default = "none")]
    gameplay_data_hash: Option<String>,

    /// ???
    #[serde(default = "none")]
    timestamp: Option<String>,

    /// ???
    #[serde(default = "none")]
    patchline_ref: Option<String>,
}


/// Generate a [Option::None] to use as default in [serde].
fn none<T>() -> Option<T> {
    Option::<T>::None
}
