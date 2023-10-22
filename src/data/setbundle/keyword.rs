//! Module defining [CardKeyword].

use crate::data::corebundle::keyword::{LocalizedCardKeyword, LocalizedCardKeywordIndex};

/// A keyword which [Card](super::card::Card)s can have.
///
/// Since more keywords will probably be added in the future, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CardKeyword {
    /// Like [Overwhelm](CardKeyword::Overwhelm), but on [Spell](super::type::CardType::Spell)s.
    ///
    /// > Inflicts damage beyond what would kill the target(s) to the enemy Nexus.
    SpellOverwhelm,

    /// [Burst](super::speed::SpellSpeed::Burst).
    ///
    /// > Can be played whenever you may act. Happens instantly and allows you to continue to play other cards.
    Burst,

    /// Countdown.
    ///
    /// > Round Start: I count down 1. At 0, activate the Countdown effect, then destroy me.
    Countdown,

    /// "Marked" play.
    ///
    /// > Get this effect when you play this unit from hand.
    #[serde(rename = "PlaySkillMark")]
    OnPlay,

    /// [Landmark](super::type::CardType::Landmark).
    ///
    /// > Landmarks take up a space on the board. They can't attack, block, or take damage.
    #[serde(rename = "LandmarkVisualOnly")]
    Landmark,

    /// ???
    Shurima,

    /// Attach.
    ///
    /// > Attach me to an ally to give it my stats and keywords while I'm attached. When that ally leaves play, Recall me.
    Attach,

    /// ???
    Noxus,

    /// Fleeting cards discard from hand when the round ends.
    Fleeting,

    /// ??????
    ClobberNoEmptySlotRequirement,

    /// Nab.
    ///
    /// > Draw a non-champion card from the bottom of the enemy deck.
    Nab,

    /// Focus.
    ///
    /// Used to disambiguate between Burst and Focus with [SpellSpeed::Burst](super::speed::SpellSpeed::Burst).
    ///
    /// > Can be played outside combat or when no other spells or skills are pending. Happens instantly and allows you to continue to play other cards.
    Focus,

    /// Enlightened.
    ///
    /// > You're Enlightened when you have 10 max mana.
    Enlightened,

    /// Invoke.
    ///
    /// > Pick a Celestial card from among 3 to create in hand.
    Invoke,

    /// Boon.
    ///
    /// > Attaches to another card in a deck. When that card is drawn, activate the effect.
    Boon,

    /// Trap.
    ///
    /// > Attaches to another card in a deck. When that card is drawn, activate the effect.
    #[serde(rename = "Autoplay")]
    Trap,

    /// Drain.
    ///
    /// > Heal your Nexus for the amount of damage dealt.
    Drain,

    /// Last Breath.
    ///
    /// > These abilities take effect when the unit dies.
    LastBreath,

    /// ???
    Demacia,

    /// ???
    BandleCity,

    /// [Fast](super::speed::SpellSpeed::Fast).
    ///
    /// > Can be played whenever you may act. Happens after your opponent has a chance to react.
    Fast,

    /// ???
    Bilgewater,

    /// ???
    Runeterra,

    /// Brash.
    ///
    /// > Can only be blocked by enemies with 3 or more Health.
    Brash,

    /// Recall.
    ///
    /// > Return a unit to hand and remove all effects applied to it.
    Recall,

    /// Weakest.
    ///
    /// > Lowest Power, with ties broken by lowest Health then lowest Cost
    Weakest,

    /// Support.
    ///
    /// > Attacking with a support unit will buff the unit to its right.
    Support,

    /// Slow.
    ///
    /// > Can be played outside of combat when no spells or skills are pending. Happens after your opponent has a chance to react.
    Slow,

    /// Obliterate.
    ///
    /// > Completely removed from the game. Doesn't cause Last Breath and can't be revived.
    Obliterate,

    /// Imbue.
    ///
    /// Currently unused.
    ///
    /// > These abilities trigger when you resolve a spell.
    Imbue,

    /// ???
    #[serde(rename = "MtTargon")]
    Targon,

    /// ???
    ShadowIsles,

    /// ??????
    AuraVisualFakeKeyword,

    /// ???
    Ionia,

    /// Nightfall.
    ///
    /// > Bonus if this is NOT the first card you play in a round.
    Nightfall,

    /// ???
    PiltoverZaun,

    /// Attune.
    ///
    /// > When I'm summoned, refill 1 spell mana.
    Attune,

    /// Daybreak.
    ///
    /// > Bonus if this is the FIRST card you play in a round.
    Daybreak,

    /// ???
    SilenceIndividualKeyword,

    /// Skill.
    ///
    /// > A unit's spell-like effect that allows enemy reactions.
    Skill,

    /// Plunder.
    ///
    /// > A card triggers its plunder ability when played if you damaged the enemy Nexus this round.
    Plunder,

    /// Double Attack.
    ///
    /// > While attacking, it strikes both before AND at the same time as its blocker.
    DoubleAttack,

    /// Vulnerable.
    ///
    /// > The enemy can challenge this unit, forcing it to block.
    Vulnerable,

    /// Elusive.
    ///
    /// > Can only be blocked by an Elusive unit.
    Elusive,

    /// Stun.
    ///
    /// > Remove a unit from combat. It can't attack or block for the rest of the round.
    Stun,

    /// Fated.
    ///
    /// > Each round, the first time an allied card targets me, grant me +1|+1.
    Fated,

    #[serde(alias = "BlocksElusive")]
    /// ???
    ///
    /// > Can block Elusive units.
    BlockElusive,

    /// Fury.
    ///
    /// > When I kill a unit, grant me +1|+1.
    Fury,

    /// Barrier.
    ///
    /// > Negates the next damage the unit would take. Lasts one round.
    Barrier,

    /// Immobile.
    ///
    /// > Can't attack or block.
    Immobile,

    /// Hallowed.
    ///
    /// > After I die, for the rest of the game when allies attack, hallow your first attacker giving it +1|+0 that round.
    Hallowed,

    /// Evolve.
    ///
    /// > I have +2|+2 once you've had Units with 6+ other positive keywords this game.
    Evolve,

    /// Frostbite.
    ///
    /// > Set a unit's Power to 0 this round. It can be changed after.
    Frostbite,

    /// Overwhelm on units.
    ///
    /// > Excess damage I deal to my blocker is dealt to the enemy Nexus.
    Overwhelm,

    /// Quick Attack.
    ///
    /// > While attacking, strikes before its blocker.
    #[serde(rename = "QuickStrike")]
    QuickAttack,

    /// Tough.
    ///
    /// > Takes 1 less damage from all sources.
    Tough,

    /// Regeneration.
    ///
    /// > Heals fully at the end of each round.
    Regeneration,

    /// Silenced.
    ///
    /// > Removes all text and keywords from a unit.
    Silenced,

    /// SpellShield.
    ///
    /// > Negates the next enemy spell or skill that would affect me.
    SpellShield,

    /// Lifesteal.
    ///
    /// > Damage this unit deals heals its Nexus that amount.
    Lifesteal,

    /// Augment.
    ///
    /// > When you play a created card, grant me +1|+0.
    Augment,

    /// Impact.
    ///
    /// > When this strikes while attacking, it deals 1 to the enemy Nexus. This keyword can stack.
    Impact,

    /// Scout.
    ///
    /// > The first time only Scout units attack each round, ready your attack.
    Scout,

    /// Ephemereal.
    ///
    /// > This unit dies when it strikes or when the round ends.
    Ephemeral,

    /// Lurk.
    ///
    /// > When you attack while I'm on top of your deck, I Lurk, granting Lurker allies everywhere +1|+0. Max once per round.
    #[serde(rename = "Lurker")]
    Lurk,

    /// Formidable.
    ///
    /// > I strike with my Health instead of my Power.
    Formidable,

    /// Challenger.
    ///
    /// > Can choose which enemy unit blocks.
    Challenger,

    /// Fearsome.
    ///
    /// > Can only be blocked by enemies with 3 or more Power.
    Fearsome,

    /// Can't Block.
    CantBlock,

    /// Deep.
    Deep,

    /// Flow.
    ///
    /// > A card activates its Flow on Round Start if you played 2+ spells or skills last round.
    Flow,

    /// Equipment.
    ///
    /// > Equip to a unit to grant the listed bonuses. If the unit leaves play, the equipment will return to your hand. You may play each equipment at most once per round.
    Equipment,

    /// Capture.
    ///
    /// > A Captured card is removed from the game. It returns when the Capturing unit leaves play.
    Capture,

    /// Unsupported card keyword.
    #[serde(other)]
    Unsupported,
}

impl CardKeyword {
    /// Get the [LocalizedCardKeyword] associated with this [CardKeyword].
    ///
    /// Returns [Option::None] if no matching [LocalizedCardKeyword] was found, for example for [CardKeyword::Unsupported] keywords.
    ///
    /// Equivalent to calling [LocalizedCardKeywordIndex::get].
    pub fn localized<'hm>(
        &self,
        hm: &'hm LocalizedCardKeywordIndex,
    ) -> Option<&'hm LocalizedCardKeyword> {
        hm.get(self)
    }

    /// Get the Discord emoji code associated with this [`CardKeyword`].
    pub fn discord_emoji(&self) -> &'static str {
        match self {
            CardKeyword::DoubleAttack => "<:doublestrike:1056023011590942770>",
            CardKeyword::Ephemeral => "<:ephemeral:1056023006876545105>",
            CardKeyword::Equipment => "<:equipment:1056022999184183317>",
            CardKeyword::Fast => "<:fast:1056022992536219728>",
            CardKeyword::Fated => "<:fated:1056022989507940414>",
            CardKeyword::Fearsome => "<:fearsome:1056022987041673367>",
            CardKeyword::Focus => "<:focus:1056022982377615390>",
            CardKeyword::Formidable => "<:formidable:1056022980007837826>",
            CardKeyword::Frostbite => "<:frostbite:1056022975436038164>",
            CardKeyword::Fury => "<:fury:1056022973636694076>",
            CardKeyword::ClobberNoEmptySlotRequirement => "",
            CardKeyword::Hallowed => "<:hallowed:1056022965914968074>",
            CardKeyword::Immobile => "<:immobile:1056022961582248026>",
            CardKeyword::Impact => "<:impact:1056022959279591485>",
            CardKeyword::Landmark => "<:landmarkvisualonly:1056022936080883783>",
            CardKeyword::LastBreath => "<:lastbreath:1056022933736263680>",
            CardKeyword::Lifesteal => "<:lifesteal:1056022931395842160>",
            CardKeyword::Lurk => "<:lurker:1056022929357426740>",
            CardKeyword::Overwhelm => "<:overwhelm:1056022921639907420>",
            CardKeyword::SpellOverwhelm => "<:overwhelm:1056022921639907420>",
            CardKeyword::QuickAttack => "<:quickstrike:1056022909535125548>",
            CardKeyword::Regeneration => "<:regeneration:1056022897396809769>",
            CardKeyword::CantBlock => "<:reckless:1056022900651602092>",
            CardKeyword::Scout => "<:scout:1056022889389903962>",
            CardKeyword::Silenced => "<:silenced:1056022882121158657>",
            CardKeyword::SilenceIndividualKeyword => "<:silenced:1056022882121158657>",
            CardKeyword::Skill => "<:skillmark:1056022880158228600>",
            CardKeyword::Slow => "<:slow:1056022877868142662>",
            CardKeyword::SpellShield => "<:spellshield:1056022875565465640>",
            CardKeyword::Stun => "<:stunned:1056022873279561759>",
            CardKeyword::Tough => "<:tough:1056022863066431591>",
            CardKeyword::Vulnerable => "<:vulnerable:1056022853411143691>",
            CardKeyword::Attach => "<:attach:1056024270712614974>",
            CardKeyword::Attune => "<:attune:1056024273401171999>",
            CardKeyword::Augment => "<:augment:1056024275628335114>",
            CardKeyword::AuraVisualFakeKeyword => "<:aura:1056024278212038756>",
            CardKeyword::Barrier => "<:barrier:1056024286177013900>",
            CardKeyword::Burst => "<:burst:1056024291457638492>",
            CardKeyword::Brash => "<:brash:1095361992547635310>",
            // CardKeyword::??? => "<:capture:1056024295190577153>",
            CardKeyword::Challenger => "<:challenger:1056024299179347988>",
            CardKeyword::Deep => "<:deep:1056024321593720923>",
            CardKeyword::Elusive => "<:elusive:1056024324110299176>",
            CardKeyword::Evolve => "<:evolve:1056024326572355654>",
            CardKeyword::Fleeting => "<:fleeting:1056024328753397862>",
            CardKeyword::Imbue => "<:imbue:1056024724314001449>",
            CardKeyword::Countdown => "<:fleeting:1056024328753397862>",  // TODO: Is this correct?
            CardKeyword::OnPlay => "<:skillmark:1056022880158228600>",
            CardKeyword::Shurima => "<:shurima:1056022884616765500>",
            CardKeyword::Noxus => "<:noxus:1056022924169064498>",
            CardKeyword::Demacia => "<:demacia:1056023014128484412>",
            CardKeyword::Runeterra => "<:runeterra:1056022895031238727>",
            CardKeyword::Targon => "<:targon:1056022866174418944>",
            CardKeyword::ShadowIsles => "<:shadowisles:1056022886848135292>",
            CardKeyword::PiltoverZaun => "<:piltoverzaun:1056022918959734835>",
            CardKeyword::Ionia => "<:ionia:1056022949569777708>",
            CardKeyword::BandleCity => "<:bandlecity:1056024280493735976>",
            CardKeyword::Bilgewater => "<:bilgewater:1056024288215437484>",
            CardKeyword::Nab => "",
            CardKeyword::Enlightened => "",
            CardKeyword::Invoke => "",
            CardKeyword::Boon => "",
            CardKeyword::Trap => "",
            CardKeyword::Drain => "",
            CardKeyword::Recall => "",
            CardKeyword::Weakest => "",
            CardKeyword::Support => "",
            CardKeyword::Obliterate => "",
            CardKeyword::Nightfall => "",
            CardKeyword::Daybreak => "",
            CardKeyword::Plunder => "",
            CardKeyword::BlockElusive => "",
            CardKeyword::Flow => "",
            CardKeyword::Capture => "<:capture:1056024295190577153>",
            CardKeyword::Unsupported => "<:invaliddeck:1056022952396730438>",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CardKeyword;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(
                    serde_json::de::from_str::<'static, CardKeyword>($src).unwrap(),
                    $res
                );
            }
        };
    }

    test_deserialization!(deserialize_spelloverwhelm, r#""SpellOverwhelm""#, CardKeyword::SpellOverwhelm);
    test_deserialization!(deserialize_burst, r#""Burst""#, CardKeyword::Burst);
    test_deserialization!(deserialize_countdown, r#""Countdown""#, CardKeyword::Countdown);
    test_deserialization!(deserialize_onplay, r#""PlaySkillMark""#, CardKeyword::OnPlay);
    test_deserialization!(deserialize_landmark, r#""LandmarkVisualOnly""#, CardKeyword::Landmark);
    test_deserialization!(deserialize_shurima, r#""Shurima""#, CardKeyword::Shurima);
    test_deserialization!(deserialize_attach, r#""Attach""#, CardKeyword::Attach);
    test_deserialization!(deserialize_noxus, r#""Noxus""#, CardKeyword::Noxus);
    test_deserialization!(deserialize_fleeting, r#""Fleeting""#, CardKeyword::Fleeting);
    test_deserialization!(deserialize_clobbernoemptyslotrequirement, r#""ClobberNoEmptySlotRequirement""#, CardKeyword::ClobberNoEmptySlotRequirement);
    test_deserialization!(deserialize_nab, r#""Nab""#, CardKeyword::Nab);
    test_deserialization!(deserialize_focus, r#""Focus""#, CardKeyword::Focus);
    test_deserialization!(deserialize_enlightened, r#""Enlightened""#, CardKeyword::Enlightened);
    test_deserialization!(deserialize_invoke, r#""Invoke""#, CardKeyword::Invoke);
    test_deserialization!(deserialize_boon, r#""Boon""#, CardKeyword::Boon);
    test_deserialization!(deserialize_trap, r#""Autoplay""#, CardKeyword::Trap);
    test_deserialization!(deserialize_drain, r#""Drain""#, CardKeyword::Drain);
    test_deserialization!(deserialize_lastbreath, r#""LastBreath""#, CardKeyword::LastBreath);
    test_deserialization!(deserialize_demacia, r#""Demacia""#, CardKeyword::Demacia);
    test_deserialization!(deserialize_bandlecity, r#""BandleCity""#, CardKeyword::BandleCity);
    test_deserialization!(deserialize_fast, r#""Fast""#, CardKeyword::Fast);
    test_deserialization!(deserialize_bilgewater, r#""Bilgewater""#, CardKeyword::Bilgewater);
    test_deserialization!(deserialize_runeterra, r#""Runeterra""#, CardKeyword::Runeterra);
    test_deserialization!(deserialize_brash, r#""Brash""#, CardKeyword::Brash);
    test_deserialization!(deserialize_recall, r#""Recall""#, CardKeyword::Recall);
    test_deserialization!(deserialize_weakest, r#""Weakest""#, CardKeyword::Weakest);
    test_deserialization!(deserialize_support, r#""Support""#, CardKeyword::Support);
    test_deserialization!(deserialize_slow, r#""Slow""#, CardKeyword::Slow);
    test_deserialization!(deserialize_obliterate, r#""Obliterate""#, CardKeyword::Obliterate);
    test_deserialization!(deserialize_imbue, r#""Imbue""#, CardKeyword::Imbue);
    test_deserialization!(deserialize_targon, r#""MtTargon""#, CardKeyword::Targon);
    test_deserialization!(deserialize_shadowisles, r#""ShadowIsles""#, CardKeyword::ShadowIsles);
    test_deserialization!(deserialize_auravisualfakekeyword, r#""AuraVisualFakeKeyword""#, CardKeyword::AuraVisualFakeKeyword);
    test_deserialization!(deserialize_ionia, r#""Ionia""#, CardKeyword::Ionia);
    test_deserialization!(deserialize_nightfall, r#""Nightfall""#, CardKeyword::Nightfall);
    test_deserialization!(deserialize_piltoverzaun, r#""PiltoverZaun""#, CardKeyword::PiltoverZaun);
    test_deserialization!(deserialize_attune, r#""Attune""#, CardKeyword::Attune);
    test_deserialization!(deserialize_daybreak, r#""Daybreak""#, CardKeyword::Daybreak);
    test_deserialization!(deserialize_silenceindividualkeyword, r#""SilenceIndividualKeyword""#, CardKeyword::SilenceIndividualKeyword);
    test_deserialization!(deserialize_skill, r#""Skill""#, CardKeyword::Skill);
    test_deserialization!(deserialize_plunder, r#""Plunder""#, CardKeyword::Plunder);
    test_deserialization!(deserialize_doubleattack, r#""DoubleAttack""#, CardKeyword::DoubleAttack);
    test_deserialization!(deserialize_vulnerable, r#""Vulnerable""#, CardKeyword::Vulnerable);
    test_deserialization!(deserialize_elusive, r#""Elusive""#, CardKeyword::Elusive);
    test_deserialization!(deserialize_stun, r#""Stun""#, CardKeyword::Stun);
    test_deserialization!(deserialize_fated, r#""Fated""#, CardKeyword::Fated);
    test_deserialization!(deserialize_blockelusive, r#""BlockElusive""#, CardKeyword::BlockElusive);
    test_deserialization!(deserialize_blockelusive2, r#""BlocksElusive""#, CardKeyword::BlockElusive);
    test_deserialization!(deserialize_fury, r#""Fury""#, CardKeyword::Fury);
    test_deserialization!(deserialize_barrier, r#""Barrier""#, CardKeyword::Barrier);
    test_deserialization!(deserialize_immobile, r#""Immobile""#, CardKeyword::Immobile);
    test_deserialization!(deserialize_hallowed, r#""Hallowed""#, CardKeyword::Hallowed);
    test_deserialization!(deserialize_evolve, r#""Evolve""#, CardKeyword::Evolve);
    test_deserialization!(deserialize_frostbite, r#""Frostbite""#, CardKeyword::Frostbite);
    test_deserialization!(deserialize_overwhelm, r#""Overwhelm""#, CardKeyword::Overwhelm);
    test_deserialization!(deserialize_quickattack, r#""QuickStrike""#, CardKeyword::QuickAttack);
    test_deserialization!(deserialize_tough, r#""Tough""#, CardKeyword::Tough);
    test_deserialization!(deserialize_regeneration, r#""Regeneration""#, CardKeyword::Regeneration);
    test_deserialization!(deserialize_silenced, r#""Silenced""#, CardKeyword::Silenced);
    test_deserialization!(deserialize_spellshield, r#""SpellShield""#, CardKeyword::SpellShield);
    test_deserialization!(deserialize_lifesteal, r#""Lifesteal""#, CardKeyword::Lifesteal);
    test_deserialization!(deserialize_augment, r#""Augment""#, CardKeyword::Augment);
    test_deserialization!(deserialize_impact, r#""Impact""#, CardKeyword::Impact);
    test_deserialization!(deserialize_scout, r#""Scout""#, CardKeyword::Scout);
    test_deserialization!(deserialize_ephemeral, r#""Ephemeral""#, CardKeyword::Ephemeral);
    test_deserialization!(deserialize_lurk, r#""Lurker""#, CardKeyword::Lurk);
    test_deserialization!(deserialize_formidable, r#""Formidable""#, CardKeyword::Formidable);
    test_deserialization!(deserialize_challenger, r#""Challenger""#, CardKeyword::Challenger);
    test_deserialization!(deserialize_fearsome, r#""Fearsome""#, CardKeyword::Fearsome);
    test_deserialization!(deserialize_cantblock, r#""CantBlock""#, CardKeyword::CantBlock);
    test_deserialization!(deserialize_deep, r#""Deep""#, CardKeyword::Deep);
    test_deserialization!(deserialize_flow, r#""Flow""#, CardKeyword::Flow);
    test_deserialization!(deserialize_equipment, r#""Equipment""#, CardKeyword::Equipment);
    test_deserialization!(deserialize_capture, r#""Capture""#, CardKeyword::Capture);
    test_deserialization!(deserialize_unsupported, r#""Xyzzy""#, CardKeyword::Unsupported);
}
