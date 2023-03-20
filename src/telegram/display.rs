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
    let title = format!("<b><u>{}</u></b>\n", escape(&card.name),);

    let stats = match &card.r#type {
        CardType::Spell => format!("{} mana\n\n", escape(&card.cost.to_string()),),
        CardType::Unit => format!(
            "{} mana {}|{}\n\n",
            escape(&card.cost.to_string()),
            escape(&card.attack.to_string()),
            escape(&card.health.to_string()),
        ),
        CardType::Landmark => format!("{} mana\n\n", &card.cost),
        _ => "".to_string(),
    };

    let set = display_set(&card.set, &globals.sets);
    let regions = display_regions(&card.regions, &globals.regions);
    let r#type = display_types(&card.r#type, &card.supertype, &card.subtypes);

    let breadcrumbs = format!("{} › {} › {}\n\n", &set, &regions, &r#type);

    let keywords = display_keywords(&card.keywords, &globals.keywords);

    let description = display_description(&card.localized_description_text);
    let levelup = display_levelup(&card.localized_levelup_text);

    let flavor = format!("<i>{}</i>\n", escape(&card.localized_flavor_text));

    let artist = format!(
        r#"<a href="{}">Illustration</a> by {}"#,
        &card
            .main_art()
            .expect("Card to have at least one illustration")
            .full_png,
        escape(&card.artist_name)
    );

    format!(
        "{}{}{}{}{}{}-----\n{}{}",
        &title, &breadcrumbs, &keywords, &stats, &description, &levelup, &flavor, &artist,
    )
}

/// Render a [CardSet] in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_set(set: &CardSet, hm: &LocalizedCardSetIndex) -> String {
    format!(
        "<i>{}</i>",
        set.localized(hm)
            .map(|o| format!("<i>{}</i>", escape(&o.name)))
            .unwrap_or_else(|| "Unknown".to_string())
    )
}

/// Render a slice of [CardRegion]s in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_regions(regions: &[CardRegion], hm: &LocalizedCardRegionIndex) -> String {
    regions
        .iter()
        .map(|region| {
            region
                .localized(hm)
                .map(|o| format!("<i>{}</i>", escape(&o.name)))
                .unwrap_or_else(|| "Unknown".to_string())
        })
        .join(", ")
}

/// Render the [CardType], the [CardSupertype] and the [CardSubtype]s in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_types(r#type: &CardType, supertype: &CardSupertype, subtypes: &[CardSubtype]) -> String {
    let mut result = String::new();

    result.push_str(
        match supertype {
            CardSupertype::Champion => "<i>Champion</i> › ",
            CardSupertype::Unsupported => "<i>Unknown</i> › ",
            _ => "",
        }
    );

    result.push_str(&format!("<i>{}</i>", escape(&String::from(r#type)),));

    if !subtypes.is_empty() {
        result.push_str(&format!(
            " › {}",
            subtypes
                .iter()
                .map(|subtype| format!("<i>{}</i>", escape(subtype)))
                .join(", ")
        ))
    }

    result
}

/// Render a slice of [CardKeyword]s in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_keywords(keywords: &[CardKeyword], hm: &LocalizedCardKeywordIndex) -> String {
    format!(
        "{}\n",
        keywords
            .iter()
            .map(|keyword| keyword
                .localized(hm)
                .map(|o| format!("[<b>{}</b>]", escape(&o.name)))
                .unwrap_or_else(|| "Unknown".to_string()))
            .join(" ")
    )
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
        tags.push("#Standard");
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
