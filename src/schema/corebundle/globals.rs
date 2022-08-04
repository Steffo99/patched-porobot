use super::keyword::CoreKeyword;
use super::rarity::CoreRarity;
use super::region::CoreRegion;
use super::speed::CoreSpellSpeed;
use super::vocabterm::CoreVocabTerm;

/// A complete `globals.json` file.
///
/// It contains a list of all vocabulary terms, [CardKeyword]s, [CardRegion]s, [SpellSpeed]s, and [CardRarity]s present in the game.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CoreGlobals {
    /// A [Vec] of all [CoreVocabTerm]s in the game.
    #[serde(rename = "vocabTerms")]
    pub vocab_terms: Vec<CoreVocabTerm>,

    /// A [Vec] of all [CoreKeyword]s in the game.
    pub keywords: Vec<CoreKeyword>,

    /// A [Vec] of all [CoreRegion]s in the game.
    pub regions: Vec<CoreRegion>,

    /// A [Vec] of all [CoreSpellSpeed]s in the game.
    #[serde(rename = "spellSpeeds")]
    pub spell_speeds: Vec<CoreSpellSpeed>,

    /// A [Vec] of all [CoreRarity]s in the game.
    pub rarities: Vec<CoreRarity>,
}