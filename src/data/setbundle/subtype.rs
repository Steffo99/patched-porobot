//! Module defining [CardSubtype].

/// A subtype of a [Card](super::card::Card), such as *Poro* or *Yordle*.
///
/// Capitalization of the various subtypes is inconsistent.
///
/// TODO: As soon as all subtypes are known, make this a enum.
pub type CardSubtype = String;
