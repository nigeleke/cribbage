use strum::IntoEnumIterator;

use super::*;

#[test]
fn new_has_face_and_suit() {
    let card = Card::new(Face::Queen, Suit::Hearts);
    assert_eq!(card.face(), Face::Queen);
    assert_eq!(card.suit(), Suit::Hearts);
}

#[test]
fn all_contains_52_cards() {
    assert_eq!(Card::all().len(), 52);
}

#[test]
fn all_contains_every_face_and_suit_combination() {
    use strum::IntoEnumIterator;

    let cards = Card::all();
    Suit::iter().for_each(|s| Face::iter().for_each(|f| assert!(cards.contains(&Card::new(f, s)))));
}

#[test]
fn rank_is_from_face() {
    Face::iter().for_each(|f| assert_eq!(Card::new(f, Suit::Hearts).rank(), f.rank()));
}

#[test]
fn value_is_from_face() {
    Face::iter().for_each(|f| assert_eq!(Card::new(f, Suit::Hearts).value(), f.value()));
}
