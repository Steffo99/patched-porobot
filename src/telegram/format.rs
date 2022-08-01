use itertools::Itertools;
use crate::data::schema::{Card, CardType};


pub fn format_card(card: &Card) -> String {
    match card.card_type {
        CardType::Spell => format!(
            "{}{}{}{}\n{}\n{}",
            format_name(&card.name),
            format_types(&card.supertype, &card.subtypes),
            format_keywords(&card.keywords),
            format_mana(card.cost),
            format_description(&card.description_raw, &card.levelup_description_raw),
            format_flavor(&card.assets.get(0).expect("card to have at least one asset").full_absolute_path, &card.flavor_text),
        ),
        CardType::Unit => format!(
            "{}{}{}{}{}\n{}\n{}",
            format_name(&card.name),
            format_types(&card.supertype, &card.subtypes),
            format_keywords(&card.keywords),
            format_mana(card.cost),
            format_stats(card.attack, card.health),
            format_description(&card.description_raw, &card.levelup_description_raw),
            format_flavor(&card.assets.get(0).expect("card to have at least one asset").full_absolute_path, &card.flavor_text),
        ),
        CardType::Ability => format!(
            "{}{}{}\n{}\n{}",
            format_name(&card.name),
            format_types(&card.supertype, &card.subtypes),
            format_keywords(&card.keywords),
            format_description(&card.description_raw, &card.levelup_description_raw),
            format_flavor(&card.assets.get(0).expect("card to have at least one asset").full_absolute_path, &card.flavor_text),
        ),
        CardType::Landmark => format!(
            "{}{}{}{}\n{}\n{}",
            format_name(&card.name),
            format_types(&card.supertype, &card.subtypes),
            format_keywords(&card.keywords),
            format_mana(card.cost),
            format_description(&card.description_raw, &card.levelup_description_raw),
            format_flavor(&card.assets.get(0).expect("card to have at least one asset").full_absolute_path, &card.flavor_text),
        ),
        CardType::Trap => format!(
            "{}{}{}\n{}\n{}",
            format_name(&card.name),
            format_types(&card.supertype, &card.subtypes),
            format_keywords(&card.keywords),
            format_description(&card.description_raw, &card.levelup_description_raw),
            format_flavor(&card.assets.get(0).expect("card to have at least one asset").full_absolute_path, &card.flavor_text),
        ),
    }
}


fn escape(s: &str) -> String {
    s
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}


fn format_name(name: &String) -> String {
    format!("<b><u>{}</u></b>\n", escape(&name))
}


fn format_types(supertype: &String, subtypes: &Vec<String>) -> String {
    let mut types = vec![];
    if supertype != "" {
        types.push(supertype.to_owned())
    }
    let mut source = subtypes.to_owned();
    types.append(&mut source);

    let mut types = types.iter()
        .map(|t| escape(&t))
        .map(|k| k.to_lowercase());

    let typestring = types.join(", ");
    if typestring == "" {
        return String::from("")
    }
    else {
        return format!("<i>{}</i>\n", &typestring)
    }
}


fn format_mana(cost: i8) -> String {
    format!("{} mana\n", cost)
}


fn format_stats(attack: i8, health: i8) -> String {
    format!("{}|{}\n", attack, health)
}


fn format_keywords(keywords: &Vec<String>) -> String {
    if keywords.len() == 0 {
        return String::from("")
    }
    else {
        format!(
            "{}\n",
            keywords.iter()
                .map(|k| escape(&k))
                .map(|k| k.to_lowercase())
                .map(|k| format!("[{}]", &k))
                .join(" ")
        )
    }
}


fn format_description(desc: &String, levelup: &String) -> String {
    if levelup == "" {
        if desc == "" {
            String::from("")
        }
        else {
            format!("{}\n", escape(&desc))
        }
    }
    else {
        format!("{}\n\n<u>Level up</u>: {}\n", escape(&desc), escape(&levelup))
    }
}


fn format_flavor(full_art: &String, flavor: &String) -> String {
    format!(r#"<a href="{}"><i>{}</i></a>"#, &full_art, escape(&flavor))
}