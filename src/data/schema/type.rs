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

    #[test]
    fn deserialize_spell() {
        assert_eq!(serde_json::de::from_str::<'static, CardType>("Spell").unwrap(), CardType::Spell);
    }

    #[test]
    fn deserialize_unit() {
        assert_eq!(serde_json::de::from_str::<'static, CardType>("Unit").unwrap(), CardType::Unit);
    }

    #[test]
    fn deserialize_ability() {
        assert_eq!(serde_json::de::from_str::<'static, CardType>("Ability").unwrap(), CardType::Ability);
    }

    #[test]
    fn deserialize_landmark() {
        assert_eq!(serde_json::de::from_str::<'static, CardType>("Landmark").unwrap(), CardType::Landmark);
    }

    #[test]
    fn deserialize_trap() {
        assert_eq!(serde_json::de::from_str::<'static, CardType>("Trap").unwrap(), CardType::Trap);
    }

    #[test]
    fn deserialize_fallback() {
        assert_eq!(serde_json::de::from_str::<'static, CardType>("Xyzzy").unwrap(), CardType::Unsupported);
    }
}