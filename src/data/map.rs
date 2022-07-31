use std::collections::HashMap;
use crate::data::schema::Card;


/// Build a [HashMap] mapping card codes strings to [Card] structs.
pub fn build_card_code_hashmap(cards: &Vec<Card>) -> HashMap<String, Card> {
    let mut map = HashMap::<String, Card>::new();

    for card in cards {
        map.insert(card.card_code.to_owned(), card.to_owned());
    }

    map
}
