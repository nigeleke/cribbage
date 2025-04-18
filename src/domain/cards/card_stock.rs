use super::card::Card;
use super::value::Value;

use crate::display::format_vec;

use serde::{Deserialize, Serialize};

use std::fmt::Display;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CardStock<T: Clone> {
    pub cards: Vec<Card>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Clone> CardStock<T> {
    pub fn remove(&mut self, card: Card) {
        self.cards.retain(|c| *c != card)
    }

    pub fn remove_all(&mut self, cards: &[Card]) {
        for card in cards {
            self.remove(*card)
        }
    }

    pub fn add(&mut self, cards: &[Card]) {
        for card in cards {
            self.cards.push(*card)
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn value(&self) -> Value {
        self.cards.iter().map(|c| c.value()).sum()
    }

    pub fn get(&self, indices: &[usize]) -> Vec<Card> {
        Vec::from_iter(indices.iter().map(|i| self.cards[*i]))
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    pub fn contains_all(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| self.contains(c))
    }

    pub fn contains_none(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| !self.cards.contains(c))
    }
}

impl<T: Clone> Default for CardStock<T> {
    fn default() -> Self {
        Self {
            cards: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<T: Clone> Display for CardStock<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", format_vec(&self.cards))
    }
}

impl<T: Clone> From<Vec<Card>> for CardStock<T> {
    fn from(value: Vec<Card>) -> Self {
        Self {
            cards: value,
            _marker: Default::default(),
        }
    }
}

impl<T: Clone> AsRef<[Card]> for CardStock<T> {
    fn as_ref(&self) -> &[Card] {
        &self.cards
    }
}

impl<T: Clone> From<&str> for CardStock<T> {
    fn from(value: &str) -> Self {
        let card_chunks = |cards: &str| {
            cards
                .chars()
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
        };

        let cards = card_chunks(value)
            .iter()
            .map(|cid| Card::from(cid.as_str()))
            .collect::<Vec<_>>();

        Self {
            cards,
            _marker: Default::default(),
        }
    }
}

impl<U: Clone> FromIterator<Card> for CardStock<U> {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        Self {
            cards: Vec::from_iter(iter),
            _marker: Default::default(),
        }
    }
}
