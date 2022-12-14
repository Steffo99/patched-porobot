//! Module defining [CardArt].

use lazy_static::lazy_static;
use regex::Regex;

/// The illustration of a [Card](super::card::Card), also referred to as an *art asset*.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CardArt {
    /// URL to the `.png` image of the rendered card.
    ///
    /// ## Example
    ///
    /// ```text
    /// https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001.png
    /// ```
    ///
    #[serde(rename = "gameAbsolutePath")]
    pub card_png: String,

    /// URL to the `.png` image of the full card art.
    ///
    /// ## Example
    ///
    /// ```text
    /// https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001-full.png
    /// ```
    ///
    #[serde(rename = "fullAbsolutePath")]
    pub full_png: String,
}

impl CardArt {
    /// URL to the `.jpg` image of the `en_us` locale of the rendered card, via my custom S3 mirror.
    ///
    /// # Example
    ///
    /// ```text
    /// https://objectstorage.eu-milan-1.oraclecloud.com/n/axxdmk4y92aq/b/porobot-storage/o/set1-en_us/en_us/img/cards/01DE001.jpg
    /// ```
    ///
    pub fn card_jpg(&self) -> String {
        lazy_static! {
            static ref GET_JPG: Regex = Regex::new(
                r#"https?://dd[.]b[.]pvp[.]net/[^/]+/(?P<bundle>[^/]+)/(?P<locale>[^/]+)/img/cards/(?P<code>.+)[.]png$"#
            )
            .unwrap();
        }

        GET_JPG
            .replace_all(
                &self.card_png,
                "https://objectstorage.eu-milan-1.oraclecloud.com/n/axxdmk4y92aq/b/porobot-storage/o/$bundle-$locale/$locale/img/cards/$code.jpg",
            )
            .to_string()
    }

    /// URL to the `.jpg` image of the `en_us` locale  of the full card art, via my custom S3 mirror.
    ///
    /// # Example
    ///
    /// ```text
    /// https://objectstorage.eu-milan-1.oraclecloud.com/n/axxdmk4y92aq/b/porobot-storage/o/set1-en_us/en_us/img/cards/01DE001-full.jpg
    /// ```
    ///
    pub fn full_jpg(&self) -> String {
        lazy_static! {
            static ref GET_JPG: Regex = Regex::new(
                r#"https?://dd[.]b[.]pvp[.]net/[^/]+/(?P<bundle>[^/]+)/(?P<locale>[^/]+)/img/cards/(?P<code>.+)[.]png$"#
            )
            .unwrap();
        }

        GET_JPG
            .replace_all(
                &self.full_png,
                "https://objectstorage.eu-milan-1.oraclecloud.com/n/axxdmk4y92aq/b/porobot-storage/o/$bundle-$locale/$locale/img/cards/$code.jpg",
            )
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::CardArt;

    #[test]
    fn deserialize() {
        assert_eq!(serde_json::de::from_str::<'static, CardArt>(r#"{"gameAbsolutePath": "https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001.png", "fullAbsolutePath": "https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001-full.png"}"#).unwrap(), CardArt { card_png: String::from("https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001.png"), full_png: String::from("https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001-full.png") });
    }

    #[test]
    fn png_to_jpg() {
        let art = CardArt {
            card_png: String::from("https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001.png"),
            full_png: String::from(
                "https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001-full.png",
            ),
        };

        assert_eq!(
            art.card_jpg(),
            "https://objectstorage.eu-milan-1.oraclecloud.com/n/axxdmk4y92aq/b/porobot-storage/o/set1-en_us/en_us/img/cards/01DE001.jpg"
        );
        assert_eq!(
            art.full_jpg(),
            "https://objectstorage.eu-milan-1.oraclecloud.com/n/axxdmk4y92aq/b/porobot-storage/o/set1-en_us/en_us/img/cards/01DE001-full.jpg"
        );
    }
}
