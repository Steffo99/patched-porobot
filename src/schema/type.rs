//! Module defining [Type].


/// A possible card type.
///
/// Since more types might be added in the future, as it happened with landmarks, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CardType {
    /// A spell.
    Spell,
    /// An unit: either a minion, or a champion.
    /// Champions have their `supertype` set to `Champion`, and their `rarity` set to `Champion` as well.
    Unit,
    /// An ability triggered by an unit.
    Ability,
    /// A landmark.
    Landmark,
    /// A trap or boon.
    Trap,

    /// Unsupported card type.
    #[serde(other)]
    Unsupported,
}


#[cfg(test)]
mod tests {
    use super::CardType;
    
    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(serde_json::de::from_str::<'static, CardType>($src).unwrap(), $res);
            }
        }
    }

    test_deserialization!(deserialize_spell, r#""Spell""#, CardType::Spell);
    test_deserialization!(deserialize_unit, r#""Unit""#, CardType::Unit);
    test_deserialization!(deserialize_ability, r#""Ability""#, CardType::Ability);
    test_deserialization!(deserialize_landmark, r#""Landmark""#, CardType::Landmark);
    test_deserialization!(deserialize_fallback, r#""Xyzzy""#, CardType::Unsupported);
}