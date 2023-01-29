//! Module defining [CardSupertype].

/// A supertype of a [Card](super::card::Card), such as *Champion*.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CardSupertype {
    /// A [Champion](super::rarity::CardRarity::Champion) card.
    Champion,
    /// An unknown supertype.
    Unsupported,
}

impl From<&str> for CardSupertype {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "champion" => CardSupertype::Champion,
            _          => CardSupertype::Unsupported,
        }
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

    test_deserialization!(deserialize_champion_lowercase, r#""champion""#, CardSupertype::Champion);
    test_deserialization!(deserialize_champion_uppercase, r#""CHAMPION""#, CardSupertype::Champion);
    test_deserialization!(deserialize_champion_titlecase, r#""Champion""#, CardSupertype::Champion);
    test_deserialization!(deserialize_unsupported, r#""sUs""#, CardSupertype::Unsupported);
}
