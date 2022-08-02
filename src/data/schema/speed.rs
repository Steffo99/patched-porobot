//! Module defining [SpellSpeed].

/// A possible spell speed.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum SpellSpeed {
    /// Non-spell cards have this speed.
    #[serde(alias = "")]
    None,
    /// A Slow spell.
    Slow,
    /// A Fast spell.
    Fast,
    /// Either a Burst or a Focus spell; to disambiguate between the two, check for the `Focus` keyword.
    Burst,
}