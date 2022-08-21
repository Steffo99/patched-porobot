//! Module defining [`DeckCodeFormat`].


/// The format of a deck code.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeckCodeFormat {
    /// The only format specified so far.
    F1,
}


impl TryFrom<u8> for DeckCodeFormat {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::F1),
            _ => Err(())
        }
    }
}

impl From<DeckCodeFormat> for u8 {
    fn from(value: DeckCodeFormat) -> Self {
        match value {
            DeckCodeFormat::F1 => 1,
        }
    }
}