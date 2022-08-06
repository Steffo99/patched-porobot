//! This module defines [CoreVocabTerm].

use std::collections::HashMap;
use std::hash::Hash;

/// A Legends of Runeterra vocabulary term, and its associated localization.
///
/// Vocabulary terms are used in [XML tags of card descriptions](crate::data::setbundle::card::Card::localized_description_xml) to provide tooltips about game mechanics.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct LocalizedVocabTerm {
    /// The internal name used by the vocabulary term.
    #[serde(rename = "nameRef")]
    pub vocabterm: String,

    /// The localized name of the vocabulary term.
    pub name: String,

    /// The description of the vocabulary term, the text of the in-game tooltip.
    pub description: String,
}

/// How [LocalizedVocabTerm]s appear in `global.json` files.
pub type LocalizedVocabTermVec = Vec<LocalizedVocabTerm>;
/// An index of [LocalizedVocabTerm]s, with [LocalizedVocabTerm::vocabterm]s as keys.
pub type LocalizedVocabTermIndex = HashMap<String, LocalizedVocabTerm>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, LocalizedVocabTerm>(r#"
                {
                    "description": "When you summon this, it gets its allegiance bonus if the top card of your deck matches its region.",
                    "name": "Allegiance",
                    "nameRef": "Allegiance"
                }
            "#).unwrap(),
            LocalizedVocabTerm {
                vocabterm: "Allegiance".to_string(),
                name: "Allegiance".to_string(),
                description: "When you summon this, it gets its allegiance bonus if the top card of your deck matches its region.".to_string(),
            }
        );
    }
}
