use glob::glob;
use patched_porobot::data::deckcode::deck::Deck;
use patched_porobot::data::setbundle::card::{Card, CardIndex};
use patched_porobot::data::setbundle::SetBundle;


fn create_index() -> CardIndex {
    let setpaths = glob("./data/set*-*")
        .expect("setglob to be a valid glob")
        .into_iter()
        .filter(|sp| sp.is_ok())
        .map(|sp| sp.unwrap());
    let mut cards: Vec<Card> = vec![];
    for setpath in setpaths {
        let set = SetBundle::load(&setpath).expect(&*format!(
            "to be able to load {:?} as a set bundle",
            &setpath
        ));
        let mut setcards = set.cards;
        cards.append(&mut setcards);
    }

    let mut index = CardIndex::new();
    for card in cards {
        index.insert(card.code.clone(), card);
    }
    index
}


macro_rules! test_legality {
    ( $id:ident, $deck:expr, $check:path, $assert:expr ) => {
        #[test]
        fn $id() {
            // FIXME: ugh
            let index = create_index();
            let deck = Deck::from_code($deck).expect("a valid deck code");
            let result = $check(&deck, &index, &regions);
            assert_eq!(result, $assert);
        }
    };
}

test_legality!(test_legality_standard_lonelyporo1, "CEAAAAIBAEAQQ", Deck::is_standard, false);
test_legality!(test_legality_standard_twistedshrimp, "CICACBAFAEBAGBQICABQCBJLF4YQOAQGAQEQYEQUDITAAAIBAMCQO", Deck::is_standard, true);
test_legality!(test_legality_standard_poros, "CQDQCAQBAMAQGAICAECACDYCAECBIFYCAMCBEEYCAUFIYANAAEBQCAIICA2QCAQBAEVTSAA", Deck::is_standard, true);
test_legality!(test_legality_standard_sand, "CMBAGBAHANTXEBQBAUCAOFJGFIYQEAIBAUOQIBAHGM5HM6ICAECAOOYCAECRSGY", Deck::is_standard, true);

test_legality!(test_legality_singleton_lonelyporo1, "CEAAAAIBAEAQQ", Deck::is_singleton, false);
test_legality!(test_legality_singleton_twistedshrimp, "CICACBAFAEBAGBQICABQCBJLF4YQOAQGAQEQYEQUDITAAAIBAMCQO", Deck::is_singleton, false);
test_legality!(test_legality_singleton_poros, "CQDQCAQBAMAQGAICAECACDYCAECBIFYCAMCBEEYCAUFIYANAAEBQCAIICA2QCAQBAEVTSAA", Deck::is_singleton, false);
test_legality!(test_legality_singleton_sand, "CMBAGBAHANTXEBQBAUCAOFJGFIYQEAIBAUOQIBAHGM5HM6ICAECAOOYCAECRSGY", Deck::is_singleton, false);
test_legality!(test_legality_singleton_paltri, "CQAAADABAICACAIFBLAACAIFAEHQCBQBEQBAGBADAQBAIAIKBUBAKBAWDUBQIBACA4GAMAIBAMCAYHJBGADAMBAOCQKRMKBLA4AQIAQ3D4QSIKZYBACAODJ3JRIW3AABQIAYUAI", Deck::is_singleton, true);
