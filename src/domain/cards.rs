use crate::domain::prelude::*;

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Cards<T> {
    cards: Vec<Card>,
    _marker: std::marker::PhantomData<T>
}

impl<T> Cards<T> {
    pub(crate) fn remove(&mut self, card: &Card) {
        self.cards.retain(|c| c != card)
    }

    pub(crate) fn remove_all(&mut self, cards: &[Card]) {
        for card in cards {
            self.remove(card)
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

    #[cfg(test)]
    pub(crate) fn get(&self, indices: &[usize]) -> Vec<Card> {
        Vec::from_iter(indices.into_iter().filter_map(|i| Some(self.cards[*i])))
    }

    #[cfg(test)]
    pub(crate) fn contains_all(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| self.cards.contains(c))
    }

    #[cfg(test)]
    pub(crate) fn contains_none(&self, cards: &[Card]) -> bool {
        cards.iter().all(|c| !self.cards.contains(c))
    }
}

impl<T> Default for Cards<T> {
    fn default() -> Self {
        Self { cards: Default::default(), _marker: Default::default() }
    }
}

impl<T> Display for Cards<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = self.cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", "); 
        write!(f, "{}", formatted)
    }
}

impl<T> From<Vec<Card>> for Cards<T> {
    fn from(value: Vec<Card>) -> Self {
        Self { cards: value, _marker: Default::default() }
    }
}

/// A deck of cards.
#[derive(Clone, Debug, PartialEq)]
pub struct DeckType;
pub type Deck = Cards<DeckType>;

impl Deck {
    pub(crate) fn shuffled_pack() -> Deck {
        let mut cards = Card::all();
        cards.shuffle(&mut thread_rng());
        cards.into()
    }

    pub(crate) fn cut(&self) -> (Card, Deck) {
        let Some((card, remainder)) = self.cards.split_first() else { unreachable!() };
        (*card, Deck::from(Vec::from(remainder)))
    }

    pub(crate) fn deal(&self, players: &HashSet<Player>) -> (HashMap<Player, Hand>, Deck) {
        let cards = &self.cards;
        let hands = players
            .iter()
            .enumerate()
            .map(|(i, p)| (*p, Hand::from(Vec::from(&cards[i*CARDS_DEALT_PER_HAND .. (i+1)*CARDS_DEALT_PER_HAND]))));
        let deck = Deck::from(Vec::from(&cards[NUMBER_OF_PLAYERS_IN_GAME * CARDS_DEALT_PER_HAND ..]));
        (HashMap::from_iter(hands), deck)
    }
}

/// A player's hand.
#[derive(Clone, Debug, PartialEq)]
pub struct HandType;
pub type Hand = Cards<HandType>;

/// The current Crib.
#[derive(Clone, Debug)]
pub struct CribType;
pub type Crib = Cards<CribType>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_52_cards() {
        let deck = Deck::shuffled_pack();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn contains_all_cards_for_all_suits_and_faces() {
        let deck = Deck::shuffled_pack();
        let cards = Card::all();

        for card in cards {
            assert!(deck.cards.contains(&card))
        }
    }

    #[test]
    fn allow_a_random_card_to_be_cut() {
        let deck = Deck::shuffled_pack();
        let (cut, remaining) = deck.cut();
        assert!(deck.cards.contains(&cut));
        assert!(!remaining.cards.contains(&cut));
        assert_eq!(remaining.cards.len(), 51);
        for card in deck.cards {
            assert_eq!(remaining.cards.contains(&card), card != cut)
        }
    }

}