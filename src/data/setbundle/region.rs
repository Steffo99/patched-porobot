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

    /// Runeterra: Ryze.
    Ryze,
    /// Runeterra: Jhin.
    Jhin,
    /// Runeterra: Varus.
    Varus,
    /// Runeterra: Aatrox.
    Aatrox,
    /// Runeterra: Neeko.
    Neeko,
    /// Runeterra: Jax.
    Jax,
    /// Runeterra: Kayn.
    Kayn,
    /// Runeterra: Poro King.
    PoroKing,
    /// Runeterra: Evelynn.
    Evelynn,
    /// Runeterra: Bard.
    Bard,


    /// Unsupported region.
    #[serde(other)]
    Unsupported,
}

impl CardRegion {
    /// Get the [`LocalizedCardRegion`] associated with this [`CardRegion`].
    ///
    /// Returns [`None`] if no matching [`LocalizedCardRegion`] was found, for example for [`CardRegion::Unsupported`] regions.
    ///
    /// Equivalent to calling [LocalizedCardRegionIndex::get].
    pub fn localized<'hm>(
        &self,
        hm: &'hm LocalizedCardRegionIndex,
    ) -> Option<&'hm LocalizedCardRegion> {
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
    /// If the region has no short code, it will return [`None`].
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

    /// Get the long code of this [`CardRegion`].
    ///
    /// If the region has no short code, it will return [`None`].
    ///
    /// Used for deck hashtags in the Telegram bot.
    pub fn to_tag(&self) -> Option<&'static str> {
        match self {
            Self::Demacia => Some("Demacia"),
            Self::Freljord => Some("Freljord"),
            Self::Ionia => Some("Ionia"),
            Self::Noxus => Some("Noxus"),
            Self::PiltoverZaun => Some("PiltoverZaun"),
            Self::ShadowIsles => Some("ShadowIsles"),
            Self::Bilgewater => Some("Bilgewater"),
            Self::Shurima => Some("Shurima"),
            Self::Targon => Some("Targon"),
            Self::BandleCity => Some("BandleCity"),
            Self::Runeterra => Some("Runeterra"),
            _ => None,
        }
    }

    /// Get the human friendly

    /// Get the Discord emoji code associated with this [`CardRegion`].
    pub fn discord_emoji(&self) -> &'static str {
        match self {
            CardRegion::Noxus => "<:noxus:1056022924169064498>",
            CardRegion::Demacia => "<:demacia:1056023014128484412>",
            CardRegion::Freljord => "<:freljord:1056024331437735936>",
            CardRegion::ShadowIsles => "<:shadowisles:1056022886848135292>",
            CardRegion::Targon => "<:targon:1056022866174418944>",
            CardRegion::Ionia => "<:ionia:1056022949569777708>",
            CardRegion::Bilgewater => "<:bilgewater:1056024288215437484>",
            CardRegion::Shurima => "<:shurima:1056022884616765500>",
            CardRegion::PiltoverZaun => "<:piltoverzaun:1056022918959734835>",
            CardRegion::BandleCity => "<:bandlecity:1056024280493735976>",
            CardRegion::Runeterra => "<:runeterra:1056022895031238727>",
            CardRegion::Ryze => "",
            CardRegion::Jhin => "",
            CardRegion::Varus => "",
            CardRegion::Aatrox => "",
            CardRegion::Neeko => "",
            CardRegion::Jax => "",
            CardRegion::Kayn => "",
            CardRegion::PoroKing => "",
            CardRegion::Evelynn => "",
            CardRegion::Bard => "",
            CardRegion::Unsupported => "<:invaliddeck:1056022952396730438>",
        }
    }
}

/// Get the [`CardRegion`] from its internal id.
///
/// If no region has the specified id, this will return [`CardRegion::Unsupported`].
impl From<u32> for CardRegion {
    fn from(value: u32) -> Self {
        match value {
            0 => CardRegion::Demacia,
            1 => CardRegion::Freljord,
            2 => CardRegion::Ionia,
            3 => CardRegion::Noxus,
            4 => CardRegion::PiltoverZaun,
            5 => CardRegion::ShadowIsles,
            6 => CardRegion::Bilgewater,
            7 => CardRegion::Shurima,
            9 => CardRegion::Targon,
            10 => CardRegion::BandleCity,
            12 => CardRegion::Runeterra,
            _ => CardRegion::Unsupported,
        }
    }
}

/// Get the internal id of this [`CardRegion`].
///
/// If the region has no internal id, it will return [`Err`].
impl TryFrom<CardRegion> for u32 {
    type Error = ();

    fn try_from(value: CardRegion) -> Result<Self, Self::Error> {
        match value {
            CardRegion::Demacia => Ok(0),
            CardRegion::Freljord => Ok(1),
            CardRegion::Ionia => Ok(2),
            CardRegion::Noxus => Ok(3),
            CardRegion::PiltoverZaun => Ok(4),
            CardRegion::ShadowIsles => Ok(5),
            CardRegion::Bilgewater => Ok(6),
            CardRegion::Shurima => Ok(7),
            CardRegion::Targon => Ok(9),
            CardRegion::BandleCity => Ok(10),
            CardRegion::Runeterra => Ok(12),
            _ => Err(()),
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
    test_deserialization!(deserialize_runeterra_ryze, r#""Ryze""#, CardRegion::Ryze);
    test_deserialization!(deserialize_runeterra_jhin, r#""Jhin""#, CardRegion::Jhin);
    test_deserialization!(deserialize_runeterra_varus, r#""Varus""#, CardRegion::Varus);
    test_deserialization!(deserialize_runeterra_aatrox, r#""Aatrox""#, CardRegion::Aatrox);
    test_deserialization!(deserialize_runeterra_neeko, r#""Neeko""#, CardRegion::Neeko);
    test_deserialization!(deserialize_runeterra_jax, r#""Jax""#, CardRegion::Jax);
    test_deserialization!(deserialize_runeterra_kayn, r#""Kayn""#, CardRegion::Kayn);
    test_deserialization!(deserialize_runeterra_poroking, r#""PoroKing""#, CardRegion::PoroKing);
    test_deserialization!(deserialize_runeterra_evelynn, r#""Evelynn""#, CardRegion::Evelynn);
    test_deserialization!(deserialize_runeterra_bard, r#""Bard""#, CardRegion::Bard);
    test_deserialization!(deserialize_fallback, r#""Xyzzy""#, CardRegion::Unsupported);
}
