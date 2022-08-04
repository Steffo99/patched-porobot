//! Module defining [SpellSpeed].


use std::collections::HashMap;

use crate::schema::corebundle::CoreSpellSpeed;

/// A possible spell speed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SpellSpeed {
    /// Non-spell cards have this speed.
    #[serde(rename = "")]
    None,
    /// A Slow spell.
    Slow,
    /// A Fast spell.
    Fast,
    /// Either a Burst or a Focus spell; to disambiguate between the two, check for the `Focus` keyword.
    Burst,
}


impl SpellSpeed {
    /// Get localized text about the speed from [crate::schema::corebundle] data.
    ///
    /// Returns `None` if no matching [CoreSpellSpeed] was found, for example if the hashmap is incomplete.
    pub fn localized<'hm>(&self, hm: &'hm HashMap<SpellSpeed, CoreSpellSpeed>) -> Option<&'hm CoreSpellSpeed> {
        hm.get(&self)
    }
}


#[cfg(test)]
mod tests {
    use super::SpellSpeed;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(serde_json::de::from_str::<'static, SpellSpeed>($src).unwrap(), $res);
            }
        }
    }

    test_deserialization!(deserialize_none, r#""""#, SpellSpeed::None);
    test_deserialization!(deserialize_slow, r#""Slow""#, SpellSpeed::Slow);
    test_deserialization!(deserialize_fast, r#""Fast""#, SpellSpeed::Fast);
    test_deserialization!(deserialize_burst, r#""Burst""#, SpellSpeed::Burst);

    #[test]
    fn deserialize_fallback() {
        assert!(serde_json::de::from_str::<'static, SpellSpeed>("Xyzzy").is_err());
    }
}