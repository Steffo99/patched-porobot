//! Module defining [CardRegion].

use crate::data::corebundle::region::{LocalizedCardRegion, LocalizedCardRegionIndex};

/// A region to which [Card](super::card::Card)s can belong to.
///
/// Since more regions might be added in the future, especially Origin ones, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CardRegion {
    /// Noxus.
    Noxus,
    /// Demacia.
    Demacia,
    /// Freljord.
    Freljord,
    /// Shadow Isles.
    ShadowIsles,
    /// Targon.
    Targon,
    /// Ionia.
    Ionia,
    /// Bilgewater.
    Bilgewater,
    /// Shurima.
    Shurima,
    /// Piltover & Zaun.
    PiltoverZaun,
    /// Bandle City.
    BandleCity,

    /// Runeterra.
    Runeterra,

    /// Origin: The Virtuoso.
    Jhin,
    /// Origin: Agony's Embrace.
    Evelynn,
    /// Origin: The Wandering Caretaker.
    Bard,

    /// Unsupported region.
    #[serde(other)]
    Unsupported,
}

impl CardRegion {
    /// Get the [LocalizedCardRegion] associated with this [CardRegion].
    ///
    /// Returns [Option::None] if no matching [LocalizedCardRegion] was found, for example for [CardRegion::Unsupported] regions.
    ///
    /// Equivalent to calling [LocalizedCardRegionIndex::get].
    pub fn localized<'hm>(&self, hm: &'hm LocalizedCardRegionIndex) -> Option<&'hm LocalizedCardRegion> {
        hm.get(self)
    }
    
    /// Get the [`CardRegion`] from its short code.
    /// 
    /// If no region is matched, will return [Option::None].
    fn from_code(value: &[char; 2]) -> Option<Self> {
        match value {
            ['D', 'E'] => Some(Self::Demacia),
            ['F', 'R'] => Some(Self::Freljord),
            ['I', 'O'] => Some(Self::Ionia),
            ['N', 'X'] => Some(Self::Noxus),
            ['P', 'Z'] => Some(Self::PiltoverZaun),
            ['S', 'I'] => Some(Self::ShadowIsles),
            ['B', 'W'] => Some(Self::Bilgewater),
            ['S', 'H'] => Some(Self::Shurima),
            ['M', 'T'] => Some(Self::Targon),
            ['B', 'C'] => Some(Self::BandleCity),
            ['R', 'U'] => Some(Self::Runeterra),
            _ => None,
        }
    }
}


#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::CardRegion;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(serde_json::de::from_str::<'static, CardRegion>($src).unwrap(), $res);
            }
        }
    }

    test_deserialization!(deserialize_noxus, r#""Noxus""#, CardRegion::Noxus);
    test_deserialization!(deserialize_demacia, r#""Demacia""#, CardRegion::Demacia);
    test_deserialization!(deserialize_freljord, r#""Freljord""#, CardRegion::Freljord);
    test_deserialization!(deserialize_shadowisles, r#""ShadowIsles""#, CardRegion::ShadowIsles);
    test_deserialization!(deserialize_targon, r#""Targon""#, CardRegion::Targon);
    test_deserialization!(deserialize_ionia, r#""Ionia""#, CardRegion::Ionia);
    test_deserialization!(deserialize_bandlecity, r#""Bilgewater""#, CardRegion::Bilgewater);
    test_deserialization!(deserialize_shurima, r#""Shurima""#, CardRegion::Shurima);
    test_deserialization!(deserialize_piltoverzaun, r#""PiltoverZaun""#, CardRegion::PiltoverZaun);
    test_deserialization!(deserialize_bandlecity, r#""BandleCity""#, CardRegion::BandleCity);
    test_deserialization!(deserialize_runeterra, r#""Runeterra""#, CardRegion::Runeterra);
    test_deserialization!(deserialize_jhin, r#""Jhin""#, CardRegion::Jhin);
    test_deserialization!(deserialize_evelynn, r#""Evelynn""#, CardRegion::Evelynn);
    test_deserialization!(deserialize_bard, r#""Bard""#, CardRegion::Bard);
    test_deserialization!(deserialize_fallback, r#""Xyzzy""#, CardRegion::Unsupported);
}
