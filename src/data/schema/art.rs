//! Module defining [CardArt].

/// An art asset associated with a [Card].
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CardArt {
    /// URL to the `.png` image of the rendered card.
    ///
    /// # Example
    ///
    /// `https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001.png`
    ///
    #[serde(rename = "gameAbsolutePath")]
    pub card_png: String,

    /// URL to the `.png` image of the full card art.
    ///
    /// # Example
    ///
    /// `https://dd.b.pvp.net/latest/set1/en_us/img/cards/01DE001-full.png`
    ///
    #[serde(rename = "fullAbsolutePath")]
    pub full_png: String,
}


impl CardArt {
    /// URL to the `.jpg` image of the rendered card, via `poro.steffo.eu`.
    ///
    /// Please do not overload this endpoint, as it currently does not use a CDN!
    ///
    /// # Example
    ///
    /// `https://poro.steffo.eu/set1-en_us/en_us/img/cards/01DE001.jpg`
    ///
    pub fn card_jpg(&self) -> String {
        self.card_png
            .replace("https://dd.b.pvp.net/latest/set1", "https://poro.steffo.eu/set1-en_us")
            .replace(".png", ".jpg")
    }

    /// URL to the `.jpg` image of the full card art, via `poro.steffo.eu`.
    ///
    /// Please do not overload this endpoint, as it currently does not use a CDN!
    ///
    /// # Example
    ///
    /// `https://poro.steffo.eu/set1-en_us/en_us/img/cards/01DE001-full.jpg`
    ///
    pub fn full_jpg(&self) -> String {
        self.full_png
            .replace("https://dd.b.pvp.net/latest/set1", "https://poro.steffo.eu/set1-en_us")
            .replace(".png", ".jpg")
    }
}