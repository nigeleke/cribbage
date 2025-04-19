use rand::{seq::SliceRandom, thread_rng};

use super::{
    card::Card,
    card_stock::CardStock,
    cut::Cut,
    hand::{Hand, Hands},
};
use crate::{constants::*, domain::Players};

/// A deck of cards.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeckType;
pub type Deck = CardStock<DeckType>;

pub trait HasDeck {
    fn deck(&self) -> &Deck;
}

impl Deck {
    pub fn shuffled_pack() -> Self {
        let mut cards = Card::all();
        cards.shuffle(&mut thread_rng());
        cards.into()
    }

    pub fn cut(&mut self) -> Cut {
        self.cards.pop().expect(stringify!(Deck::cut))
    }

    pub fn deal(&mut self, players: &Players) -> Hands {
        let hands = Hands::from_iter(players.iter().enumerate().map(|(i, p)| {
            let start = i * CARDS_DEALT_PER_HAND;
            let end = start + CARDS_DEALT_PER_HAND;
            (*p, Hand::from(self.cards[start..end].to_vec()))
        }));

        self.cards.drain(..players.len() * CARDS_DEALT_PER_HAND);

        hands
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
        let deck0 = Deck::shuffled_pack();
        let mut deck1 = deck0.clone();
        let cut = deck1.cut();

        assert!(deck0.cards.contains(&cut));
        assert!(!deck1.cards.contains(&cut));
        assert_eq!(deck1.cards.len(), 51);

        for card in deck0.cards {
            assert_eq!(deck1.cards.contains(&card), card != cut)
        }
    }
}
