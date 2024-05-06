use crate::domain::prelude::*;

use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

/// A deck of cards.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub(crate) fn shuffled_pack() -> Deck {
        let mut cards = Card::all();
        cards.shuffle(&mut thread_rng());
        Deck(cards)
    }

    pub(crate) fn cut(&self) -> (Card, Deck) {
        let Some((card, remainder)) = self.0.split_first() else { unreachable!() };
        (*card, Deck(remainder.into()))
    }

    pub(crate) fn deal(&self, players: &HashSet<Player>) -> (HashMap<Player, Hand>, Deck) {
        let cards = &self.0;
        let hands = players
            .into_iter()
            .enumerate()
            .map(|(i, p)| (*p, Hand(Vec::from(&cards[i*CARDS_DEALT_PER_HAND .. (i+1)*CARDS_DEALT_PER_HAND]))));
        let deck = Deck(Vec::from(&cards[NUMBER_OF_PLAYERS_IN_GAME * CARDS_DEALT_PER_HAND ..]));
        (HashMap::from_iter(hands), deck)
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Deck({:?})", self.0)
    }
}

/// A player's hand.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Hand(Vec<Card>);

/// The current Crib.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Crib(Vec<Card>);

impl Crib {
    pub(crate) fn new() -> Self {
        Self(vec![])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_52_cards() {
        let deck = Deck::shuffled_pack();
        assert_eq!(deck.0.len(), 52);
    }

    #[test]
    fn contains_all_cards_for_all_suits_and_faces() {
        let deck = Deck::shuffled_pack();
        let cards = Card::all();

        for card in cards {
            assert!(deck.0.contains(&card))
        }
    }

    #[test]
    fn allow_a_random_card_to_be_cut() {
        let deck = Deck::shuffled_pack();
        let (cut, remaining) = deck.cut();
        assert!(deck.0.contains(&cut));
        assert!(!remaining.0.contains(&cut));
        assert_eq!(remaining.0.len(), 51);
        for card in deck.0 {
            assert_eq!(remaining.0.contains(&card), card != cut)
        }
    }

}