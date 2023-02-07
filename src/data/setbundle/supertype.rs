//! Module defining [CardSupertype].

/// A supertype of a [`Card`](super::card::Card), such as *Champion*.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CardSupertype {
    /// No supertype, like most cards in the game.
    #[serde(rename = "")]
    None,
    /// A [Champion](super::rarity::CardRarity::Champion).
    #[serde(alias = "Champion")]
    Champion,
    /// A supertype of an unknown type.
    #[serde(other)]
    Unsupported,
}

impl From<&CardSupertype> for &'static str {
    fn from(cs: &CardSupertype) -> Self {
        match cs {
            CardSupertype::None => "",
            CardSupertype::Champion => "Champion",
            CardSupertype::Unsupported => "Unknown",
        }
    }
}

impl From<&CardSupertype> for String {
    fn from(cs: &CardSupertype) -> Self {
        <&CardSupertype as Into<&'static str>>::into(cs).to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::CardSupertype;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(
                    serde_json::de::from_str::<'static, CardSupertype>($src).unwrap(),
                    $res
                );
            }
        };
    }

    test_deserialization!(deserialize_none, r#""""#, CardSupertype::None);
    test_deserialization!(deserialize_champion_uppercase, r#""CHAMPION""#, CardSupertype::Champion);
    test_deserialization!(deserialize_champion_titlecase, r#""Champion""#, CardSupertype::Champion);
    test_deserialization!(deserialize_unsupported, r#""sUs""#, CardSupertype::Unsupported);
}
