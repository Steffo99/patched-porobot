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
    /// If no region has the specified short code, this will return [`CardRegion::Unsupported`].
    pub fn from_code(value: &str) -> Self {
        match value {
            "DE" => Self::Demacia,
            "FR" => Self::Freljord,
            "IO" => Self::Ionia,
            "NX" => Self::Noxus,
            "PZ" => Self::PiltoverZaun,
            "SI" => Self::ShadowIsles,
            "BW" => Self::Bilgewater,
            "SH" => Self::Shurima,
            "MT" => Self::Targon,
            "BC" => Self::BandleCity,
            "RU" => Self::Runeterra,
            _ => Self::Unsupported,
        }
    }

    /// Get the short code of this [`CardRegion`].
    ///
    /// If the region has no short code, it will return [`Option::None`].
    pub fn to_code(&self) -> Option<String> {
        match self {
            Self::Demacia => Some("DE".to_string()),
            Self::Freljord => Some("FR".to_string()),
            Self::Ionia => Some("IO".to_string()),
            Self::Noxus => Some("NX".to_string()),
            Self::PiltoverZaun => Some("PZ".to_string()),
            Self::ShadowIsles => Some("SI".to_string()),
            Self::Bilgewater => Some("BW".to_string()),
            Self::Shurima => Some("SH".to_string()),
            Self::Targon => Some("MT".to_string()),
            Self::BandleCity => Some("BC".to_string()),
            Self::Runeterra => Some("RU".to_string()),
            _ => None,
        }
    }

    /// Get the [`CardRegion`] from its internal id.
    ///
    /// If no region has the specified id, this will return [`CardRegion::Unsupported`].
    pub fn from_id(value: u32) -> Self {
        match value {
            0 => Self::Demacia,
            1 => Self::Freljord,
            2 => Self::Ionia,
            3 => Self::Noxus,
            4 => Self::PiltoverZaun,
            5 => Self::ShadowIsles,
            6 => Self::Bilgewater,
            7 => Self::Shurima,
            9 => Self::Targon,
            10 => Self::BandleCity,
            12 => Self::Runeterra,
            _ => Self::Unsupported,
        }
    }

    /// Get the internal id of this [`CardRegion`].
    ///
    /// If the region has no internal id, it will return [`Option::None`].
    pub fn to_id(&self) -> Option<u32> {
        match self {
            Self::Demacia => Some(0),
            Self::Freljord => Some(1),
            Self::Ionia => Some(2),
            Self::Noxus => Some(3),
            Self::PiltoverZaun => Some(4),
            Self::ShadowIsles => Some(5),
            Self::Bilgewater => Some(6),
            Self::Shurima => Some(7),
            Self::Targon => Some(9),
            Self::BandleCity => Some(10),
            Self::Runeterra => Some(12),
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
    test_deserialization!(deserialize_bilgewater, r#""Bilgewater""#, CardRegion::Bilgewater);
    test_deserialization!(deserialize_shurima, r#""Shurima""#, CardRegion::Shurima);
    test_deserialization!(deserialize_piltoverzaun, r#""PiltoverZaun""#, CardRegion::PiltoverZaun);
    test_deserialization!(deserialize_bandlecity, r#""BandleCity""#, CardRegion::BandleCity);
    test_deserialization!(deserialize_runeterra, r#""Runeterra""#, CardRegion::Runeterra);
    test_deserialization!(deserialize_jhin, r#""Jhin""#, CardRegion::Jhin);
    test_deserialization!(deserialize_evelynn, r#""Evelynn""#, CardRegion::Evelynn);
    test_deserialization!(deserialize_bard, r#""Bard""#, CardRegion::Bard);
    test_deserialization!(deserialize_fallback, r#""Xyzzy""#, CardRegion::Unsupported);
}
