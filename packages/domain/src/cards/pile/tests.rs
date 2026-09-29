use crate::{card, cards};

use super::*;

#[derive(Debug)]
struct TestPileType {}
type TestPile = Pile<TestPileType>;
impl TestPile {}

#[test]
fn default_pile_will_be_empty() {
    let pile = TestPile::default();
    assert!(pile.is_empty());
    assert_eq!(pile.len(), 0);
}

#[test]
fn created_pile_as_content() {
    let pile = TestPile::from(cards!("QH"));
    assert!(!pile.is_empty());
    assert_eq!(pile.len(), 1);
}

#[test]
fn can_test_for_card_in_pile() {
    let pile = TestPile::from(cards!("AH2C3D4S"));
    assert!(pile.contains(&card!("AH")));
    assert!(!pile.contains(&card!("QH")));
}

#[test]
fn can_test_for_all_cards_in_pile() {
    let pile = TestPile::from(cards!("AH2C3D4S"));
    assert!(pile.contains_all(&cards!("AH2C3D")));
    assert!(!pile.contains_all(&cards!("AH2CQH4S")));
}

#[test]
fn can_test_for_no_cards_in_pile() {
    let pile = TestPile::from(cards!("AH2C3D4S"));
    assert!(!pile.contains_none(&cards!("AH2C3D")));
    assert!(pile.contains_none(&cards!("QHQCQHQS")));
}

#[test]
fn can_add_card_to_pile() {
    let mut pile = TestPile::default();
    pile.add(card!("AH"));
    assert!(pile.contains(&card!("AH")));
}

#[test]
fn can_add_cards_to_pile() {
    let mut pile = TestPile::default();
    pile.add_all(&cards!("AH2H"));
    assert!(pile.contains_all(&cards!("AH2H")));
}

#[test]
fn can_remove_card_from_pile() {
    let mut pile = TestPile::from(cards!("AH2H"));
    pile.remove(card!("AH"));
    assert!(!pile.contains(&card!("AH")));
    assert!(pile.contains(&card!("2H")));
}

#[test]
fn can_remove_cards_from_pile() {
    let mut pile = TestPile::from(cards!("AH2H3H"));
    pile.remove_all(&cards!("AH2H"));
    assert!(pile.contains_none(&cards!("AH2H")));
    assert!(pile.contains(&card!("3H")));
}

#[test]
fn can_shuffle_a_pile() {
    let mut rng = rand::rng();
    let cards = cards!("AH2H3H4H5H6H7H8H9HTHJHQHKHAC2C3C4C5C6C7C8C9CTCJCQCKC");
    let cards_len = cards.len();
    let pile = TestPile::from(cards.clone()).shuffled(&mut rng);

    assert_eq!(pile.len(), cards_len);
    assert!(pile.contains_all(&cards));
}

#[test]
fn can_sort_a_pile() {
    let mut rng = rand::rng();
    let cards = cards!("AH2H3H4H5H6H7H8H9HTHJHQHKHAC2C3C4C5C6C7C8C9CTCJCQCKC");
    let cards_len = cards.len();
    let pile = TestPile::from(cards.clone()).shuffled(&mut rng).sorted();

    assert_eq!(pile.len(), cards_len);
    assert!(pile.contains_all(&cards));
    pile.iter()
        .zip(cards!(
            "KCKHQCQHJCJHTCTH9C9H8C8H7C7H6C6H5C5H4C4H3C3H2C2HACAH"
        ))
        .for_each(|(actual, expected)| assert!(*actual == expected));
}

#[test]
fn has_debug_text() {
    let cards = cards!("AH2H3H4H");
    let pile = TestPile::from(cards);
    let actual = format!("{pile:?}");
    let expected = "[AH, 2H, 3H, 4H]";
    assert_eq!(actual, expected);
}
