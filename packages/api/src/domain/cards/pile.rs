use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Card;
use crate::display::format_vec;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pile<T: Clone> {
    cards: Vec<Card>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Clone> Pile<T> {
    pub const fn len(&self) -> usize {
        self.cards.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

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
        let keep = self.cards.split_off(n);
        let taken = self.cards.clone();
        self.cards = keep;
        taken
    }

    pub fn contains(&self, card: Card) -> bool {
        self.cards.contains(&card)
    }

    pub fn contains_all(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| self.cards.contains(c))
    }

    pub fn contains_none(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| !self.cards.contains(c))
    }

    pub fn sort_by_rank(&mut self) {
        self.cards.sort_by_key(Card::rank);
    }
}

impl<T: Clone> Default for Pile<T> {
    fn default() -> Self {
        Self {
            cards: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<T: Clone> From<&[Card]> for Pile<T> {
    fn from(value: &[Card]) -> Self {
        Self {
            cards: Vec::from(value),
            _marker: Default::default(),
        }
    }
}

impl<T: Clone> FromIterator<Card> for Pile<T> {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        Self {
            cards: Vec::from_iter(iter),
            _marker: Default::default(),
        }
    }
}

impl<T: Clone> AsRef<[Card]> for Pile<T> {
    fn as_ref(&self) -> &[Card] {
        &self.cards
    }
}

impl<'a, T: Clone> IntoIterator for &'a Pile<T> {
    type Item = &'a Card;
    type IntoIter = std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}

impl<T: Clone> Display for Pile<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        let short_name = type_name.rsplit("::").next().unwrap_or("Cards");
        let display_name = short_name.strip_suffix("Type").unwrap_or(short_name);
        if display_name.starts_with("Any") {
            write!(f, "[{}]", format_vec(&self.cards))
        } else {
            write!(f, "{display_name}({})", format_vec(&self.cards))
        }
    }
}

#[cfg(test)]
#[coverage(off)]
mod test {
    use std::str::FromStr;

    use super::*;
    use crate::{card, cards, pile};

    impl<T: Clone> FromStr for Pile<T> {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.len().is_multiple_of(2) {
                let card_chunks = |cards: &str| {
                    cards
                        .chars()
                        .collect::<Vec<_>>()
                        .chunks(2)
                        .map(|chunk| chunk.iter().collect::<String>())
                        .collect::<Vec<_>>()
                };

                let cards = card_chunks(s)
                    .iter()
                    .flat_map(|cid| Card::from_str(cid.as_str()))
                    .collect::<Vec<_>>();

                Ok(Self {
                    cards,
                    _marker: Default::default(),
                })
            } else {
                Err("invalid string length for cards".into())
            }
        }
    }

    #[derive(Clone)]
    struct TestType {}

    #[derive(Clone)]
    struct OtherTestType {}

    #[derive(Clone)]
    struct SourceType {}

    #[derive(Clone)]
    struct TargetType {}

    #[test]
    fn default_pile_will_be_empty() {
        let pile = Pile::<TestType>::default();
        assert!(pile.is_empty());
        assert_eq!(pile.len(), 0);
    }

    #[test]
    fn created_pile_as_content() {
        let pile = pile!(TestType, "QH");
        assert!(!pile.is_empty());
        assert_eq!(pile.len(), 1);
    }

    #[test]
    fn cards_can_be_displayed() {
        let pile = pile!(TestType, "AH2C3D4S");
        insta::assert_snapshot!(pile.to_string(), @"Test(AH, 2C, 3D, 4S)");
    }

    #[test]
    fn generic_cards_can_be_displayed() {
        #[derive(Clone)]
        struct AnyType {}
        let pile = pile!(AnyType, "AH2C3D4S");
        insta::assert_snapshot!(pile.to_string(), @"[AH, 2C, 3D, 4S]");
    }

    #[test]
    fn can_test_for_card_in_pile() {
        let pile = pile!(TestType, "AH2C3D4S");
        assert!(pile.contains(card!("AH")));
        assert!(!pile.contains(card!("QH")));
    }

    #[test]
    fn can_test_for_all_cards_in_pile() {
        let pile = pile!(TestType, "AH2C3D4S");
        assert!(pile.contains_all(&cards!("AH2C3D")));
        assert!(!pile.contains_all(&cards!("AH2CQH4S")));
    }

    #[test]
    fn can_test_for_no_cards_in_pile() {
        let pile = pile!(TestType, "AH2C3D4S");
        assert!(!pile.contains_none(&cards!("AH2C3D")));
        assert!(pile.contains_none(&cards!("QHQCQHQS")));
    }
}
