use strum::IntoEnumIterator;

use crate::constants::STANDARD_DECK_SIZE;

use super::*;

#[test]
fn new_has_face_and_suit() {
    let card = Card::new(Face::Queen, Suit::Hearts);
    assert_eq!(card.face(), Face::Queen);
    assert_eq!(card.suit(), Suit::Hearts);
}

#[test]
fn all_contains_52_cards() {
    assert_eq!(Card::all().len(), STANDARD_DECK_SIZE);
}

#[test]
fn all_contains_every_face_and_suit_combination() {
    use strum::IntoEnumIterator;

    let cards = Card::all();
    Suit::iter().for_each(|s| Face::iter().for_each(|f| assert!(cards.contains(&Card::new(f, s)))));
}

#[test]
fn faces_have_rank() {
    Face::iter().for_each(|f| assert_eq!(Card::new(f, Suit::Hearts).rank(), f.rank()));
}

#[test]
fn faces_have_value() {
    Face::iter().for_each(|f| assert_eq!(Card::new(f, Suit::Hearts).value(), f.value()));
}

#[test]
fn debug_text_is_card_short_text() {
    use crate::card;
    let cards = Card::all();
    let debug_text = cards.iter().map(|c| format!("{c:?}"));
    cards
        .iter()
        .zip(debug_text)
        .for_each(|(card, text)| assert_eq!(card, &card!(&text)));
}

#[test]
fn muliple_cards_can_be_formatted_as_short_text() {
    let cards = Card::all().into_iter().take(4).collect::<Vec<_>>();
    let actual = cards_to_string(&cards);
    let expected = "AH, 2H, 3H, 4H";
    assert_eq!(actual, expected);
}
