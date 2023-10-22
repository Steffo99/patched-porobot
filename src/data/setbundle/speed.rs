//! Module defining [SpellSpeed].

use crate::data::corebundle::speed::{LocalizedSpellSpeed, LocalizedSpellSpeedIndex};

/// A possible [`Spell`](super::type::CardType::Spell) speed.
#[non_exhaustive]
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
    /// Unsupported spell speed.
    #[serde(other)]
    Unsupported,
}

impl SpellSpeed {
    /// Get the [`LocalizedSpellSpeed`] associated with this [`SpellSpeed`].
    ///
    /// Returns [`None`] if no matching [`LocalizedSpellSpeed`] was found, for example spell speeds missing from the index.
    ///
    /// Equivalent to calling [`LocalizedSpellSpeedIndex::get`].
    pub fn localized<'hm>(
        &self,
        hm: &'hm LocalizedSpellSpeedIndex,
    ) -> Option<&'hm LocalizedSpellSpeed> {
        hm.get(self)
    }
}

#[cfg(test)]
mod tests {
    use super::SpellSpeed;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(
                    serde_json::de::from_str::<'static, SpellSpeed>($src).unwrap(),
                    $res
                );
            }
        };
    }

    test_deserialization!(deserialize_none, r#""""#, SpellSpeed::None);
    test_deserialization!(deserialize_slow, r#""Slow""#, SpellSpeed::Slow);
    test_deserialization!(deserialize_fast, r#""Fast""#, SpellSpeed::Fast);
    test_deserialization!(deserialize_burst, r#""Burst""#, SpellSpeed::Burst);
    test_deserialization!(deserialize_unsupported, r#""Xyzzy""#, SpellSpeed::Unsupported);
}
