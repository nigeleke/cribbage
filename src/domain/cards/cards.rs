use super::card::Card;

use crate::types::prelude::*;
use crate::fmt::format_vec;

use serde::{Deserialize, Serialize};

use std::fmt::Display;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cards<T>
where T: Clone {
    pub(crate) cards: Vec<Card>,
    _marker: std::marker::PhantomData<T>
}

impl<T> Cards<T>
where T: Clone {
    pub(crate) fn remove(&mut self, card: Card) {
        self.cards.retain(|c| *c != card)
    }

    pub(crate) fn remove_all(&mut self, cards: &[Card]) {
        for card in cards {
            self.remove(*card)
        }
    }

    pub(crate) fn add(&mut self, cards: &[Card]) {
        for card in cards {
            self.cards.push(*card)
        }
    }

    pub(crate) fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    pub(crate) fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    #[cfg(test)]
    pub(crate) fn get(&self, indices: &[usize]) -> Vec<Card> {
        Vec::from_iter(indices.into_iter().filter_map(|i| Some(self.cards[*i])))
    }

    pub(crate) fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    #[cfg(test)]
    pub(crate) fn contains_all(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| self.contains(c))
    }

    #[cfg(test)]
    pub(crate) fn contains_none(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| !self.cards.contains(c))
    }
}

impl<T> HasValue for Cards<T>
where T: Clone {
    fn value(&self) -> Value {
        self.cards.iter().map(|&c| c.value()).sum()
    }
}

impl<T> Default for Cards<T>
where T: Clone {
    fn default() -> Self {
        Self { cards: Default::default(), _marker: Default::default() }
    }
}

impl<T> Display for Cards<T>
where T: Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_vec(&self.cards))
    }
}

impl<T> From<Vec<Card>> for Cards<T>
where T: Clone {
    fn from(value: Vec<Card>) -> Self {
        Self { cards: value, _marker: Default::default() }
    }
}

#[cfg(test)]
impl<T> From<&str> for Cards<T>
where T: Clone {
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

        Self { cards, _marker: Default::default() }
    }
}

impl<U> FromIterator<Card> for Cards<U>
where U: Clone {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        Self { cards: Vec::from_iter(iter), _marker: Default::default() }
    }
}
