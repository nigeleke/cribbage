use rand::prelude::{Rng, SliceRandom};

use crate::card::Card;

#[derive(Clone, PartialEq, Eq)]
pub struct Pile<T> {
    cards: Vec<Card>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Pile<T> {
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_all(&mut self, cards: &[Card]) {
        self.cards.extend(cards);
    }

    pub fn remove(&mut self, card: Card) {
        self.cards.retain(|c| c != &card);
    }

    pub fn remove_all(&mut self, cards: &[Card]) {
        self.cards.retain(|c| !cards.contains(c));
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn take(&mut self, n: usize) -> Vec<Card> {
        self.cards.drain(..n).collect()
    }

    pub fn contains_all(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| self.cards.contains(c))
    }

    pub fn contains_none(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| !self.cards.contains(c))
    }

    pub fn sorted(mut self) -> Self {
        self.cards.sort_by_key(|c| (c.rank(), c.suit()));
        self.cards.reverse();
        self
    }

    pub fn shuffled(mut self, rng: &mut impl Rng) -> Self {
        self.cards.shuffle(rng);
        self
    }
}

impl<T> Default for Pile<T> {
    fn default() -> Self {
        Self {
            cards: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> From<&[Card]> for Pile<T> {
    fn from(value: &[Card]) -> Self {
        Self {
            cards: value.to_vec(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> From<Vec<Card>> for Pile<T> {
    fn from(value: Vec<Card>) -> Self {
        Self::from(value.as_slice())
    }
}

impl<T> std::ops::Deref for Pile<T> {
    type Target = [Card];

    fn deref(&self) -> &Self::Target {
        self.cards.as_slice()
    }
}

impl<T> std::fmt::Debug for Pile<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}]", crate::card::cards_to_string(&self.cards))
    }
}

#[cfg(test)]
#[coverage(off)]
mod test {
    use macros::*;

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
}
