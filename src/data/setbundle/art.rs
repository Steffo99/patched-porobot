//! Module defining [CardArt].

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

    /// Get the URL to convert the image at the given URL into JPG using imgproxy.
    #[cfg(feature = "jpg")]
    fn imgproxy_convert_to_jpg(url: &str) -> String {
        use base64::Engine;

        let url = base64::prelude::BASE64_URL_SAFE.encode(url);
        let url = format!("/{url}.jpg");

        log::trace!("Created JPG conversion URL: {url}");

        url
    }

    /// Add the HMAC required by imgproxy to authenticate the source of the image requester to the URL.
    ///
    /// # Panics
    ///
    /// If the `POROXY_KEY` or `POROXY_SALT` variables are not defined.
    ///
    #[cfg(feature = "jpg")]
    fn imgproxy_authenticate_url(url: &str) -> String {
        use base64::Engine;
        use hmac::Mac;
        use std::env;

        let key = env::var("POROXY_KEY")
            .expect("POROXY_KEY to be set");
        let key = hex::decode(key)
            .expect("POROXY_KEY to be a valid hex code");

        let salt = env::var("POROXY_SALT")
            .expect("POROXY_SALT to be set");
        let salt = hex::decode(salt)
            .expect("POROXY_SALT to be a valid hex code");
        let salt: String = String::from_utf8(salt)
            .expect("salt to be a valid UTF-8 string");

        let mut hmac = hmac::Hmac::<sha2::Sha256>::new_from_slice(key.as_slice())
            .expect("HMAC to be initialized successfully");
        hmac.update(&format!("{salt}{url}").into_bytes());
        let hmac = hmac.finalize().into_bytes();
        let hmac = base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(hmac);

        let url = format!("/{hmac}{url}");

        log::trace!("Created authenticated URL: {url}");

        url
    }

    /// URL to the `.jpg` image of the rendered card, via imgproxy.
    ///
    /// # Panics
    ///
    /// If the `POROXY_HOST`, `POROXY_KEY` and `POROXY_SALT` variables are not defined.
    ///
    #[cfg(feature = "jpg")]
    pub fn card_jpg(&self) -> String {
        use std::env;

        let host = env::var("POROXY_HOST")
            .expect("POROXY_HOST to be set");

        let url = Self::imgproxy_authenticate_url(
            &Self::imgproxy_convert_to_jpg(
                &self.card_png
            )
        );

        let url = format!("{host}{url}");
        log::trace!("Accessed card_jpg: {url}");

        url
    }

    /// URL to the `.jpg` image of the rendered card, via imgproxy.
    ///
    /// # Panics
    ///
    /// If the `POROXY_HOST`, `POROXY_KEY` and `POROXY_SALT` variables are not defined.
    ///
    #[cfg(feature = "jpg")]
    pub fn full_jpg(&self) -> String {
        use std::env;

        let host = env::var("POROXY_HOST")
            .expect("POROXY_HOST to be set");

        let url = Self::imgproxy_authenticate_url(
            &Self::imgproxy_convert_to_jpg(
                &self.full_png
            )
        );

        let url = format!("{host}{url}");
        log::trace!("Accessed full_jpg: {url}");

        url
    }
}

#[cfg(test)]
mod tests {
    use super::CardArt;

    #[test]
    fn deserialize() {
        assert_eq!(serde_json::de::from_str::<'static, CardArt>(r#"{"gameAbsolutePath": "https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001.png", "fullAbsolutePath": "https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001-full.png"}"#).unwrap(), CardArt { card_png: String::from("https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001.png"), full_png: String::from("https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001-full.png") });
    }
}
