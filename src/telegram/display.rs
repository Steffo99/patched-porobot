//! This module defines functions to convert [patched-porobot] structs to [String]s formatted with [Telegram Bot HTML](https://core.telegram.org/bots/api#html-style).
//!
//! TODO: Add support for non-latin languages.
//!
//! TODO: Preferably refactor everything in here, as the code is poor quality.

use std::collections::HashMap;
use itertools::Itertools;
use teloxide::utils::html::escape;


/// Render a [Card] to a [String] formatted with [Telegram Bot HTML](https://core.telegram.org/bots/api#html-style).
pub fn display_card(card: &, mg: &MappedGlobals) -> String {
    let title = format!(r#"<a href="{}"><b><i>{}</b></i></a>"#, &card.main_art().card_png, escape(&card.name));

    let stats = match &card.r#type {
        CardType::Spell => format!("{} mana", escape(&card.cost.to_string())),
        CardType::Unit => format!("{} mana {}|{}", escape(&card.cost.to_string()), escape(&card.attack.to_string()), escape(&card.health.to_string())),
        CardType::Ability => "".to_string(),
        CardType::Landmark => format!("{} mana", &card.cost),
        CardType::Trap => "".to_string(),
        CardType::Unsupported => "".to_string(),
    };

    let set = format!("<i>{}</i>", escape(&display_set(card.set, &mg.sets)));
    let regions = format!("<i>{}</i>", escape(&display_regions(&card.regions, &mg.regions)));
    let r#type = format!("<i>{}</i>", escape(&display_type(card.r#type)));

    let breadcrumbs = format!("{} › {} › {}", &set, &regions, &r#type);

    let description = card.localized_description_text.clone();
    let flavor = format!("<i>{}</i>", &card.localized_flavor_text);
    let artist = format!(r#"<a href="{}">Art by {}</a>"#, &card.main_art().full_png, &card.artist_name);

    format!(
        "{title} {stats}\n{breadcrumbs}\n\n{description}\n\n-----\n{flavor}\n\n{artist}",
        title=title,
        stats=stats,
        breadcrumbs=breadcrumbs,
        description=description,
        flavor=flavor,
        artist=artist,
    )
}


/// Render a [CardSet] to a [String].
fn display_set(set: CardSet, hm: &HashMap<CardSet, CoreSet>) -> String {
    set
        .localized(&hm)
        .map(|o| o.name.clone())
        .unwrap_or_else(|| "Unknown".to_string())
}


/// Render a slice of [CardRegion]s to a [String].
fn display_regions(regions: &[CardRegion], hm: &HashMap<CardRegion, CoreRegion>) -> String {
    regions
        .iter()
        .map(|region| region
            .localized(&hm)
            .map(|o| o.name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
        )
        .join(", ")
}


/// Render a [CardType] to a [String].
fn display_type(r#type: CardType) -> String {
    match r#type {
        CardType::Spell => "Spell",
        CardType::Unit => "Unit",
        CardType::Ability => "Ability",
        CardType::Landmark => "Landmark",
        CardType::Trap => "Trap",
        CardType::Unsupported => "Unknown",
    }.to_string()
}
