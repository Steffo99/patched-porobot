//! Module defining functions to format Legends of Runeterra data in [Telegram Bot HTML].
//!
//! [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style

use itertools::Itertools;
use teloxide::utils::html::escape;
use crate::data::setbundle::card::Card;
use crate::data::setbundle::r#type::CardType;
use crate::data::corebundle::globals::LocalizedGlobalsIndexes;
use crate::data::corebundle::keyword::LocalizedCardKeywordIndex;
use crate::data::corebundle::region::LocalizedCardRegionIndex;
use crate::data::corebundle::set::LocalizedCardSetIndex;
use crate::data::setbundle::keyword::CardKeyword;
use crate::data::setbundle::region::CardRegion;
use crate::data::setbundle::set::CardSet;
use crate::data::setbundle::subtype::CardSubtype;
use crate::data::setbundle::supertype::CardSupertype;


/// Render a [Card] in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
pub fn display_card(globals: &LocalizedGlobalsIndexes, card: &Card) -> String {
    let title = format!(
        "<b><u>{}</u></b>\n",
        escape(&card.name),
    );

    let stats = match &card.r#type {
        CardType::Spell => format!(
            "{} mana\n",
            escape(&card.cost.to_string()),
        ),
        CardType::Unit => format!(
            "{} mana {}|{}\n",
            escape(&card.cost.to_string()),
            escape(&card.attack.to_string()),
            escape(&card.health.to_string()),
        ),
        CardType::Landmark => format!(
            "{} mana\n",
            &card.cost
        ),
        _ => "".to_string(),
    };

    let set = display_set(&card.set, &globals.sets);
    let regions = display_regions(&card.regions, &globals.regions);
    let r#type = display_types(&card.r#type, &card.supertype, &card.subtypes);

    let breadcrumbs = format!("{} › {} › {}\n", &set, &regions, &r#type);

    let keywords = display_keywords(&card.keywords, &globals.keywords);

    let description = format!("{}\n", escape(&card.localized_description_text));

    let flavor = format!(
        "<i>{}</i>\n",
        escape(&card.localized_flavor_text)
    );

    let artist = format!(
        r#"<a href="{}">Illustration</a> by {}"#,
        &card.main_art().expect("Card to have at least one illustration").full_png,
        escape(&card.artist_name)
    );

    format!(
        "{title}{breadcrumbs}\n{keywords}{stats}{description}\n-----\n{flavor}{artist}",
    )
}


/// Render a [CardSet] in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_set(set: &CardSet, hm: &LocalizedCardSetIndex) -> String {
    format!(
        "<i>{}</i>",
        set
            .localized(hm)
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
        .map(|region| region
            .localized(hm)
            .map(|o| format!("<i>{}</i>", escape(&o.name)))
            .unwrap_or_else(|| "Unknown".to_string())
        )
        .join(", ")
}


/// Render the [CardType], the [CardSupertype] and the [CardSubtype]s in [Telegram Bot HTML].
///
/// [Telegram Bot HTML]: https://core.telegram.org/bots/api#html-style
fn display_types(r#type: &CardType, supertype: &CardSupertype, subtypes: &[CardSubtype]) -> String {
    let mut result = String::new();

    if supertype != "" {
        result.push_str(&*format!(
            "<i>{}</i> › ",
            escape(&supertype),
        ));
    };

    result.push_str(&*format!(
        "<i>{}</i>",
        escape(&*String::from(r#type)),
    ));

    if subtypes.len() > 0 {
        result.push_str(
            &*format!(
                " › {}",
                subtypes.iter()
                    .map(|subtype| format!("<i>{}</i>", escape(&subtype)))
                    .join(", ")
            )
        )
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
                .unwrap_or_else(|| "Unknown".to_string())
            )
            .join(" ")
    )

}
