use super::card::Card;
use super::cut::Cut;
use super::cards::Cards;
use super::hand::{Hand, Hands};

use crate::constants::*;
use crate::types::prelude::*;

use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashSet;

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

    pub(crate) fn cut(&self) -> (Cut, Deck) {
        let Some((&card, remainder)) = self.cards.split_first() else { unreachable!() };
        (card.into(), Deck::from(Vec::from(remainder)))
    }

    pub(crate) fn deal(&self, players: &HashSet<Player>) -> (Hands, Deck) {
        let cards = &self.cards;
        let hands = players
            .iter()
            .enumerate()
            .map(|(i, p)| (*p, Hand::from(Vec::from(&cards[i*CARDS_DEALT_PER_HAND .. (i+1)*CARDS_DEALT_PER_HAND]))));
        let deck = Deck::from(Vec::from(&cards[NUMBER_OF_PLAYERS_IN_GAME * CARDS_DEALT_PER_HAND ..]));
        (Hands::from_iter(hands), deck)
    }
}

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
        let cut: Card = cut.into();
        assert!(deck.contains(&cut));
        assert!(!remaining.cards.contains(&cut));
        assert_eq!(remaining.cards.len(), 51);
        for card in deck.cards {
            assert_eq!(remaining.cards.contains(&card), card != cut)
        }
    }

}