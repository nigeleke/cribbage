use constants::*;

use crate::domain::{Card, Hand, cards::pile::Pile};

/// Marker type for the main deck (the draw pile).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeckType;

/// The deck contains all cards at the start of the game and is drawn from
/// during dealing and for the starter cut.
pub type Deck = Pile<DeckType>;

/// The main draw pile / stock.
pub trait HasDeck {
    /// Immutable access to the deck.
    fn deck(&self) -> &Deck;

    /// Mutable access to the deck.
    fn deck_mut(&mut self) -> &mut Deck;
}

impl Deck {
    pub fn shuffled_pack() -> Self {
        use rand::{rng, seq::SliceRandom};

        let mut cards = Card::all();
        cards.shuffle(&mut rng());
        Self::from(cards.as_ref())
    }

    pub fn cut(&mut self) -> Card {
        self.pop().expect("available card")
    }

    pub fn deal(&mut self, count: usize) -> Vec<Hand> {
        Vec::from_iter((0..count).map(|_| {
            let cards = self.take(CARDS_DEALT_PER_HAND);
            Hand::from(cards.as_ref())
        }))
    }
}

#[cfg(test)]
pub(crate) const STANDARD_DECK_SIZE: usize = 52;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_52_cards() {
        let deck = Deck::shuffled_pack();
        assert_eq!(deck.len(), STANDARD_DECK_SIZE);
    }

    #[test]
    fn contains_all_cards_for_all_suits_and_faces() {
        let deck = Deck::shuffled_pack();
        let cards = Card::all();

        for card in cards {
            assert!(deck.contains(card))
        }
    }

    #[test]
    fn allow_a_random_card_to_be_cut() {
        let deck0 = Deck::shuffled_pack();
        let mut deck1 = deck0.clone();
        let cut = deck1.cut();

        assert!(deck0.contains(cut));
        assert!(!deck1.contains(cut));
        assert_eq!(deck1.len(), 51);
    }

    #[test]
    fn allow_deals() {
        let deck0 = Deck::shuffled_pack();
        let mut deck1 = deck0.clone();
        let deals = deck1.deal(2);
        assert!(deck0.contains_all(deals[0].as_ref()));
        assert!(deck0.contains_all(deals[1].as_ref()));
        assert!(deck1.contains_none(deals[0].as_ref()));
        assert!(deck1.contains_none(deals[1].as_ref()));
    }
}
