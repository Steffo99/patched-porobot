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


#[cfg(test)]
mod tests {
    use super::CoreVocabTerm;

    #[test]
    fn deserialize() {
        assert_eq!(
            serde_json::de::from_str::<'static, CoreVocabTerm>(r#"
                {
                    "description": "When you summon this, it gets its allegiance bonus if the top card of your deck matches its region.",
                    "name": "Allegiance",
                    "nameRef": "Allegiance"
                }
            "#).unwrap(),
            CoreVocabTerm {
                vocabterm: "Allegiance".to_string(),
                name: "Allegiance".to_string(),
                description: "When you summon this, it gets its allegiance bonus if the top card of your deck matches its region.".to_string(),
            }
        );
    }
}
