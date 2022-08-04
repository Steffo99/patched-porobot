//! This module defines [CoreVocabTerm].

/// A Legends of Runeterra vocabulary term, and its associated localization.
///
/// I'm not sure where these are used, other than in in-game tooltips.
///
/// TODO: Find out where these are used.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CoreVocabTerm {
    /// The internal name used by the vocabulary term.
    ///
    /// TODO: Map these to an enum.
    #[serde(rename = "nameRef")]
    pub vocabterm: String,

    /// The localized name of the vocabulary term.
    pub name: String,

    /// The description of the vocabulary term, the text of the in-game tooltip.
    pub description: String,
}