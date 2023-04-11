//! Module defining [CardFormat].


use crate::data::corebundle::format::{LocalizedCardFormat, LocalizedCardFormatIndex};

/// A format in which [Card](super::card::Card)s can be played in.
///
/// Since more keywords will probably be added in the future, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CardFormat {
    /// Standard.
    #[serde(rename = "client_Formats_Standard_name")]
    Standard,

    /// Singleton.
    ///
    /// Note that this format does not currently appear in card data, as it seems to be used only for the deckbuilder.
    #[serde(rename = "client_Deckbuilder_RulesFilters_Singleton")]
    Singleton,

    /// Eternal.
    #[serde(rename = "client_Formats_Eternal_name")]
    Eternal,

    /// Commons only.
    #[serde(rename = "client_Formats_CommonsOnly_name")]
    CommonsOnly,

    /// Even-cost cards only.
    #[serde(rename = "client_Formats_EvenCostCards_name")]
    EvenCostCards,

    /// Unsupported format.
    #[serde(other)]
    Unsupported,
}

impl CardFormat {
    /// Get the [LocalizedCardFormat] associated with this [CardFormat].
    ///
    /// Returns [None] if no matching [LocalizedCardFormat] was found, for example formats missing from the index.
    ///
    /// Equivalent to calling [LocalizedCardFormatIndex::get].
    pub fn localized<'hm>(
        &self,
        hm: &'hm LocalizedCardFormatIndex,
    ) -> Option<&'hm LocalizedCardFormat> {
        hm.get(self)
    }
}

#[cfg(test)]
mod tests {
    use super::CardFormat;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(
                    serde_json::de::from_str::<'static, CardFormat>($src).unwrap(),
                    $res
                );
            }
        };
    }

    test_deserialization!(deserialize_standard, r#""client_Formats_Standard_name""#, CardFormat::Standard);
    test_deserialization!(deserialize_singleton, r#""client_Deckbuilder_RulesFilters_Singleton""#, CardFormat::Singleton);
    test_deserialization!(deserialize_eternal, r#""client_Formats_Eternal_name""#, CardFormat::Eternal);
    test_deserialization!(deserialize_commonsonly, r#""client_Formats_CommonsOnly_name""#, CardFormat::CommonsOnly);
    test_deserialization!(deserialize_evencostcards, r#""client_Formats_EvenCostCards_name""#, CardFormat::EvenCostCards);
    test_deserialization!(deserialize_unsupported, r#""xyzzy""#, CardFormat::Unsupported);
}
