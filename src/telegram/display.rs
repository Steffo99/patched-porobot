//! Module defining functions to format Legends of Runeterra data in [Telegram Bot HTML].
//!
//! [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style

use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;
use crate::data::corebundle::globals::LocalizedGlobalsIndexes;
use crate::data::corebundle::keyword::LocalizedCardKeywordIndex;
use crate::data::corebundle::region::LocalizedCardRegionIndex;
use crate::data::corebundle::set::LocalizedCardSetIndex;
use crate::data::deckcode::deck::Deck;
use crate::data::setbundle::card::{Card, CardIndex};
use crate::data::setbundle::keyword::CardKeyword;
use crate::data::setbundle::r#type::CardType;
use crate::data::setbundle::region::CardRegion;
use crate::data::setbundle::set::CardSet;
use crate::data::setbundle::subtype::CardSubtype;
use crate::data::setbundle::supertype::CardSupertype;
use itertools::Itertools;
use teloxide::utils::html::escape;

/// Render a [Card] in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
pub fn display_card(globals: &LocalizedGlobalsIndexes, card: &Card) -> String {

    let title: String = display_title(&card.name);
    let r#type: String = display_type(globals, card);
    let stats = display_stats(card);
    let subtypes: String = display_subtypes(&card.subtypes);

    let header = format!("{} ({})\n{}\n{}\n", &title, &r#type, &stats, &subtypes);

    let keywords = display_keywords(&card.keywords, &globals.keywords);
    let description = display_description(&card.localized_description_text);
    let levelup = display_levelup(&card.localized_levelup_text);

    let body = format!("{}{}{}", &keywords, &description, &levelup);

    let set = display_set(&card.set, &globals.sets);
    let regions = display_regions(&card.regions, &globals.regions);
    let flavor = display_flavor(&card.main_art().expect("Card to have at least one illustration").full_png, &card.localized_flavor_text);

    let footer = format!("{}{}\n{}", &set, &regions, &flavor);

    format!("{}⸻\n\n{}⸻\n\n{}", &header, &body, &footer)
}

/// Render a [Card]'s title in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_title(name: &str) -> String {
    format!("<b><u>{}</u></b>", escape(name))
}

/// Render the [Card]'s in-game type in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_type(globals: &LocalizedGlobalsIndexes, card: &Card) -> String {
    match card.r#type {
        CardType::Spell => {
            "Spell".to_string()
        }
        CardType::Equipment => {
            CardKeyword::Equipment.localized(&globals.keywords).map(|l| l.name.clone()).unwrap_or_else(|| "EQUIPMENT".to_string())
        }
        CardType::Unit => {
            if card.supertype.eq(&CardSupertype::Champion) {
                "Champion".to_string()
            }
            else {
                "Unit".to_string()
            }
        }
        CardType::Ability => {
            if card.keywords.contains(&CardKeyword::Skill) {
                CardKeyword::Skill.localized(&globals.keywords).map(|l| l.name.clone()).unwrap_or_else(|| "SKILL".to_string())
            }
            else {
                "Ability".to_string()  // Does this exist?
            }
        }
        CardType::Landmark => {
            CardKeyword::Landmark.localized(&globals.keywords).map(|l| l.name.clone()).unwrap_or_else(|| "LANDMARK".to_string())
        }
        CardType::Trap => {
            if card.keywords.contains(&CardKeyword::Trap) {
                CardKeyword::Trap.localized(&globals.keywords).map(|l| l.name.clone()).unwrap_or_else(|| "TRAP".to_string())
            }
            else if card.keywords.contains(&CardKeyword::Boon) {
                CardKeyword::Boon.localized(&globals.keywords).map(|l| l.name.clone()).unwrap_or_else(|| "Boon".to_string())
            }
            else {
                "Trigger".to_string()  // Does this exist?
            }
        }
        CardType::Unsupported => {
            "UNKNOWN?".to_string()
        }
    }
}

fn display_stats(card: &Card) -> String {
    match &card.r#type {
        CardType::Spell => format!("{} mana", escape(&card.cost.to_string())),
        CardType::Unit => format!(
            "{} mana · {}|{}",
            escape(&card.cost.to_string()),
            escape(&card.attack.to_string()),
            escape(&card.health.to_string()),
        ),
        CardType::Landmark => format!("{} mana", &card.cost),
        _ => "".to_string(),
    }
}

/// Render the [CardSubtype]s in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_subtypes(subtypes: &[CardSubtype]) -> String {
    let result = subtypes
        .iter()
        .map(|s| titlecase(s))
        .map(|s| escape(&s))
        .map(|s| format!("<i>{s}</i>"))
        .join(", ");

    if result.is_empty() {
        result
    } else {
        result + "\n"
    }
}

/// Render a slice of [CardKeyword]s in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_keywords(keywords: &[CardKeyword], hm: &LocalizedCardKeywordIndex) -> String {
    let result = keywords
        .iter()
        .filter(|keyword| match keyword {
            CardKeyword::Countdown => false,
            CardKeyword::OnPlay => false,
            CardKeyword::Landmark => false,
            CardKeyword::Shurima => false,
            CardKeyword::Noxus => false,
            CardKeyword::ClobberNoEmptySlotRequirement => false,
            CardKeyword::Nab => false,
            CardKeyword::Enlightened => false,
            CardKeyword::Invoke => false,
            CardKeyword::Drain => false,
            CardKeyword::LastBreath => false,
            CardKeyword::Demacia => false,
            CardKeyword::BandleCity => false,
            CardKeyword::Bilgewater => false,
            CardKeyword::Runeterra => false,
            CardKeyword::Recall => false,
            CardKeyword::Weakest => false,
            CardKeyword::Support => false,
            CardKeyword::Obliterate => false,
            CardKeyword::Imbue => false,
            CardKeyword::Targon => false,
            CardKeyword::ShadowIsles => false,
            CardKeyword::AuraVisualFakeKeyword => false,
            CardKeyword::Ionia => false,
            CardKeyword::PiltoverZaun => false,
            CardKeyword::SilenceIndividualKeyword => false,
            CardKeyword::Plunder => false,
            CardKeyword::Silenced => false,
            _ => true,
        })
        .map(|keyword| keyword
            .localized(hm)
            .map(|o| format!("[<b>{}</b>: {}]\n", escape(&o.name), escape(&o.description)))
            .unwrap_or_else(|| "[<b>UNKNOWN?</b>]\n".to_string()))
        .join("");

    if result.is_empty() {
        result
    } else {
        result + "\n"
    }
}

/// Render a [Card::localized_description_text] in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_description(description: &String) -> String {
    if description.is_empty() {
        "".to_string()
    } else {
        format!("{}\n\n", escape(description))
    }
}

/// Render a [Card::localized_levelup_text] in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_levelup(levelup: &String) -> String {
    if levelup.is_empty() {
        "".to_string()
    } else {
        format!("<u>Level up</u>: {}\n\n", escape(levelup))
    }
}

/// Render a [CardSet] in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_set(set: &CardSet, hm: &LocalizedCardSetIndex) -> String {
    format!(
        "<i>{}</i>",
        set.localized(hm)
            .map(|o| format!("<i>{}</i>", escape(&o.name)))
            .unwrap_or_else(|| "UNKNOWN?".to_string())
    )
}

/// Render a slice of [CardRegion]s in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_regions(regions: &[CardRegion], hm: &LocalizedCardRegionIndex) -> String {
    let result = regions
        .iter()
        .map(|region| {
            region
                .localized(hm)
                .map(|o| format!("<u>{}</u>", escape(&o.name)))
                .unwrap_or_else(|| "UNKNOWN?".to_string())
        })
        .join(", ");

    if result.is_empty() {
        result
    } else {
        format!(" · {}", &result)
    }
}

/// Render the flavor text of a [Card] in [Telegram Bot HTML], linking to the given art.
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_flavor(art_url: &str, localized_flavor_text: &str) -> String {
    format!("<a href=\"{}\"><i>{}</i></a>", &art_url, escape(localized_flavor_text))
}

/// Render a [Deck] in [Telegram Bot HTML], with an optional `name`.
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
pub fn display_deck(index: &CardIndex, deck: &Deck, code: &str, name: &Option<&str>) -> String {
    let cards = deck
        .contents
        .keys()
        .map(|k| (
            index.get(k),
            deck.contents.get(k).expect("CardCode from Deck to have a quantity"),
        ))
        .sorted_by(|(opt_a, _), (opt_b, _)| {
            if opt_a.is_none() && opt_b.is_none() {
                return Equal;
            }
            if opt_b.is_none() {
                return Greater;
            }
            else if opt_a.is_none() {
                return Less;
            }

            let card_a = opt_a.expect("opt_a to be Some");
            let card_b = opt_b.expect("opt_b to be Some");

            card_a
                .cost
                .cmp(&card_b.cost)
                .then(card_a.name.cmp(&card_b.name))
        })
        .map(|(card, quantity)| {
            let name = match card {
                None => "<i>Unknown Card</i>".to_string(),
                Some(card) => match card.supertype {
                    CardSupertype::Champion => format!("<u>{}</u>", escape(&card.name)),
                    _                       => escape(&card.name),
                }
            };

            format!("<b>{}×</b> {}", &quantity, &name)
        })
        .join("\n");

    let mut tags: Vec<&'static str> = vec![];

    let regions = if let Some(regions) = deck.standard(index) {
        tags.push("#Standard_4_5");
        regions
    } else if let Some(regions) = deck.eternal(index) {
        tags.push("#Eternal");
        regions
    } else if let Some(regions) = deck.unlimited_champions(index) {
        tags.push("#UnlimitedChampions");
        regions
    } else if let Some(regions) = deck.singleton(index) {
        tags.push("#Singleton");
        regions
    } else {
        HashSet::new()
    };

    for region in regions {
        tags.push(match region {
            CardRegion::Noxus => "#Noxus",
            CardRegion::Demacia => "#Demacia",
            CardRegion::Freljord => "#Freljord",
            CardRegion::ShadowIsles => "#ShadowIsles",
            CardRegion::Targon => "#Targon",
            CardRegion::Ionia => "#Ionia",
            CardRegion::Bilgewater => "#Bilgewater",
            CardRegion::Shurima => "#Shurima",
            CardRegion::PiltoverZaun => "#PiltoverZaun",
            CardRegion::BandleCity => "#BandleCity",
            CardRegion::Runeterra => "#Runeterra",
            CardRegion::Unsupported => "<i>Unknown</i>",
        })
    }

    let tags = tags.join(", ");
    let tags = if !tags.is_empty() { format!("{}\n", &tags) } else { "".to_string() };

    match name {
        Some(name) => format!("<b><u>{}</u></b>\n<code>{}</code>\n{}\n{}", &name, &code, &tags, &cards),
        None => format!("<code>{}</code>\n{}\n{}", &code, &tags, &cards),
    }
}


// https://stackoverflow.com/a/38406885/4334568
fn titlecase(s: &str) -> String {
    let s = s.to_lowercase();
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}