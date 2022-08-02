//! Module defining [CardKeyword].

/// A keyword which cards can have.
///
/// Since more keywords will probably be added in the future, this enum is [non_exaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CardKeyword {
    /// Overwhelm on spells.
    ///
    /// > Inflicts damage beyond what would kill the target(s) to the enemy Nexus.
    SpellOverwhelm,

    /// [SpellSpeed::Burst].
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

    /// [CardType::Landmark].
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
    /// Used to disambiguate between Burst and Focus with [SpellSpeed::Burst].
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

    /// [SpellSpeed::Fast].
    ///
    /// > Can be played whenever you may act. Happens after your opponent has a chance to react.
    Fast,

    /// ???
    Bilgewater,

    /// ???
    Runeterra,

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

    /// Imbue, an unused keyword.
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

    /// Plunder,
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

    /// ???
    ///
    /// > Can block Elusives.
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
    /// > I have +2|+2 once you've given or summoned allies with 6+ other positive keywords this game.
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
    #[serde(rename="QuickStrike")]
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
    #[serde(rename="Lurker")]
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

    /// Unsupported card keyword.
    #[serde(other)]
    Unsupported,
}


#[cfg(test)]
mod tests {
    use super::CardKeyword;

    macro_rules! test_deserialization {
        ( $id:ident, $src:literal, $res:expr ) => {
            #[test]
            fn $id() {
                assert_eq!(serde_json::de::from_str::<'static, CardKeyword>($src).unwrap(), $res);
            }
        }
    }

    test_deserialization!(deserialize_spelloverwhelm, "SpellOverwhelm", CardKeyword::SpellOverwhelm);
    test_deserialization!(deserialize_burst, "Burst", CardKeyword::Burst);
    test_deserialization!(deserialize_countdown, "Countdown", CardKeyword::Countdown);
    test_deserialization!(deserialize_onplay, "PlaySkillMark", CardKeyword::OnPlay);
    test_deserialization!(deserialize_landmark, "LandmarkVisualOnly", CardKeyword::Landmark);
    test_deserialization!(deserialize_shurima, "Shurima", CardKeyword::Shurima);
    test_deserialization!(deserialize_attach, "Attach", CardKeyword::Attach);
    test_deserialization!(deserialize_noxus, "Noxus", CardKeyword::Noxus);
    test_deserialization!(deserialize_fleeting, "Fleeting", CardKeyword::Fleeting);
    test_deserialization!(deserialize_clobbernoemptyslotrequirement, "ClobberNoEmptySlotRequirement", CardKeyword::ClobberNoEmptySlotRequirement);
    test_deserialization!(deserialize_nab, "Nab", CardKeyword::Nab);
    test_deserialization!(deserialize_focus, "Focus", CardKeyword::Focus);
    test_deserialization!(deserialize_enlightened, "Enlightened", CardKeyword::Enlightened);
    test_deserialization!(deserialize_invoke, "Invoke", CardKeyword::Invoke);
    test_deserialization!(deserialize_boon, "Boon", CardKeyword::Boon);
    test_deserialization!(deserialize_trap, "Autoplay", CardKeyword::Trap);
    test_deserialization!(deserialize_drain, "Drain", CardKeyword::Drain);
    test_deserialization!(deserialize_lastbreath, "LastBreath", CardKeyword::LastBreath);
    test_deserialization!(deserialize_demacia, "Demacia", CardKeyword::Demacia);
    test_deserialization!(deserialize_bandlecity, "BandleCity", CardKeyword::BandleCity);
    test_deserialization!(deserialize_fast, "Fast", CardKeyword::Fast);
    test_deserialization!(deserialize_bilgewater, "Bilgewater", CardKeyword::Bilgewater);
    test_deserialization!(deserialize_runeterra, "Runeterra", CardKeyword::Runeterra);
    test_deserialization!(deserialize_recall, "Recall", CardKeyword::Recall);
    test_deserialization!(deserialize_weakest, "Weakest", CardKeyword::Weakest);
    test_deserialization!(deserialize_support, "Support", CardKeyword::Support);
    test_deserialization!(deserialize_slow, "Slow", CardKeyword::Slow);
    test_deserialization!(deserialize_obliterate, "Obliterate", CardKeyword::Obliterate);
    test_deserialization!(deserialize_imbue, "Imbue", CardKeyword::Imbue);
    test_deserialization!(deserialize_targon, "MtTargon", CardKeyword::Targon);
    test_deserialization!(deserialize_shadowisles, "ShadowIsles", CardKeyword::ShadowIsles);
    test_deserialization!(deserialize_auravisualfakekeyword, "AuraVisualFakeKeyword", CardKeyword::AuraVisualFakeKeyword);
    test_deserialization!(deserialize_ionia, "Ionia", CardKeyword::Ionia);
    test_deserialization!(deserialize_nightfall, "Nightfall", CardKeyword::Nightfall);
    test_deserialization!(deserialize_piltoverzaun, "PiltoverZaun", CardKeyword::PiltoverZaun);
    test_deserialization!(deserialize_attune, "Attune", CardKeyword::Attune);
    test_deserialization!(deserialize_daybreak, "Daybreak", CardKeyword::Daybreak);
    test_deserialization!(deserialize_silenceindividualkeyword, "SilenceIndividualKeyword", CardKeyword::SilenceIndividualKeyword);
    test_deserialization!(deserialize_skill, "Skill", CardKeyword::Skill);
    test_deserialization!(deserialize_plunder, "Plunder", CardKeyword::Plunder);
    test_deserialization!(deserialize_doubleattack, "DoubleAttack", CardKeyword::DoubleAttack);
    test_deserialization!(deserialize_vulnerable, "Vulnerable", CardKeyword::Vulnerable);
    test_deserialization!(deserialize_elusive, "Elusive", CardKeyword::Elusive);
    test_deserialization!(deserialize_stun, "Stun", CardKeyword::Stun);
    test_deserialization!(deserialize_fated, "Fated", CardKeyword::Fated);
    test_deserialization!(deserialize_blockelusive, "BlockElusive", CardKeyword::BlockElusive);
    test_deserialization!(deserialize_fury, "Fury", CardKeyword::Fury);
    test_deserialization!(deserialize_barrier, "Barrier", CardKeyword::Barrier);
    test_deserialization!(deserialize_immobile, "Immobile", CardKeyword::Immobile);
    test_deserialization!(deserialize_hallowed, "Hallowed", CardKeyword::Hallowed);
    test_deserialization!(deserialize_evolve, "Evolve", CardKeyword::Evolve);
    test_deserialization!(deserialize_frostbite, "Frostbite", CardKeyword::Frostbite);
    test_deserialization!(deserialize_overwhelm, "Overwhelm", CardKeyword::Overwhelm);
    test_deserialization!(deserialize_quickattack, "QuickStrike", CardKeyword::QuickAttack);
    test_deserialization!(deserialize_tough, "Tough", CardKeyword::Tough);
    test_deserialization!(deserialize_regeneration, "Regeneration", CardKeyword::Regeneration);
    test_deserialization!(deserialize_silenced, "Silenced", CardKeyword::Silenced);
    test_deserialization!(deserialize_spellshield, "SpellShield", CardKeyword::SpellShield);
    test_deserialization!(deserialize_lifesteal, "Lifesteal", CardKeyword::Lifesteal);
    test_deserialization!(deserialize_augment, "Augment", CardKeyword::Augment);
    test_deserialization!(deserialize_impact, "Impact", CardKeyword::Impact);
    test_deserialization!(deserialize_scout, "Scout", CardKeyword::Scout);
    test_deserialization!(deserialize_ephemeral, Ephemeral, CardKeyword::Ephemeral);
    test_deserialization!(deserialize_lurk, Lurker, CardKeyword::Lurk);
    test_deserialization!(deserialize_formidable, Formidable, CardKeyword::Formidable);
    test_deserialization!(deserialize_challenger, Challenger, CardKeyword::Challenger);
    test_deserialization!(deserialize_fearsome, Fearsome, CardKeyword::Fearsome);
    test_deserialization!(deserialize_cantblock, CantBlock, CardKeyword::CantBlock);
    test_deserialization!(deserialize_deep, Deep, CardKeyword::Deep);
    test_deserialization!(deserialize_unsupported, Unsupported, CardKeyword::Unsupported);
}
