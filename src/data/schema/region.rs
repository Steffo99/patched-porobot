//! Module defining [CardRegion].

/// A region to which cards can belong to.
///
/// Since more regions might be added in the future, especially Origin ones, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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


#[cfg(test)]
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

    test_deserialization!(deserialize_noxus, "Noxus", CardRegion::Noxus);
    test_deserialization!(deserialize_demacia, "Demacia", CardRegion::Demacia);
    test_deserialization!(deserialize_freljord, "Freljord", CardRegion::Freljord);
    test_deserialization!(deserialize_shadowisles, "ShadowIsles", CardRegion::ShadowIsles);
    test_deserialization!(deserialize_targon, "Targon", CardRegion::Targon);
    test_deserialization!(deserialize_ionia, "Ionia", CardRegion::Ionia);
    test_deserialization!(deserialize_shurima, "Shurima", CardRegion::Shurima);
    test_deserialization!(deserialize_piltoverzaun, "PiltoverZaun", CardRegion::PiltoverZaun);
    test_deserialization!(deserialize_bandlecity, "BandleCity", CardRegion::BandleCity);
    test_deserialization!(deserialize_runeterra, "Runeterra", CardRegion::Runeterra);
    test_deserialization!(deserialize_jhin, "Jhin", CardRegion::Jhin);
    test_deserialization!(deserialize_evelynn, "Evelynn", CardRegion::Evelynn);
    test_deserialization!(deserialize_bard, "Bard", CardRegion::Bard);
    test_deserialization!(deserialize_fallback, "Xyzzy", CardRegion::Unsupported);
}
