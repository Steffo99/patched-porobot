//! Module defining [CardType].

/// A possible [Card](super::card::Card) type.
///
/// Since more types might be added in the future, as it happened with landmarks, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CardType {
    /// A spell.
    Spell,

    /// An unit: either a minion, or a champion.
    ///
    /// Champions have their [supertype](super::card::Card::supertype) set to `Champion`, and their [rarity](super::card::Card::rarity) set to [CardRarity::Champion](super::rarity::CardRarity::Champion) as well.
    Unit,

    /// An ability triggered by an [Unit](CardType::Unit).
    Ability,

    /// A landmark.
    Landmark,

    /// An autoplaying card: either a trap or boon.
    ///
    /// Disambiguate between the two using [CardKeyword::Trap](super::keyword::CardKeyword::Trap) and [CardKeyword::Boon](super::keyword::CardKeyword::Boon).
    Trap,

    /// Unsupported card type.
    #[serde(other)]
    Unsupported,
}

impl From<&CardType> for &'static str {
    fn from(r#type: &CardType) -> Self {
        match r#type {
            CardType::Spell => "Spell",
            CardType::Unit => "Unit",
            CardType::Ability => "Ability",
            CardType::Landmark => "Landmark",
            CardType::Trap => "Trap",
            CardType::Unsupported => "Unknown",
        }
    }
}

impl From<&CardType> for String {
    fn from(cs: &CardType) -> Self {
        <&CardType as Into<&'static str>>::into(cs).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::CardType;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(
                    serde_json::de::from_str::<'static, CardType>($src).unwrap(),
                    $res
                );
            }
        };
    }

    test_deserialization!(deserialize_spell, r#""Spell""#, CardType::Spell);
    test_deserialization!(deserialize_unit, r#""Unit""#, CardType::Unit);
    test_deserialization!(deserialize_ability, r#""Ability""#, CardType::Ability);
    test_deserialization!(deserialize_landmark, r#""Landmark""#, CardType::Landmark);
    test_deserialization!(deserialize_fallback, r#""Xyzzy""#, CardType::Unsupported);
}
