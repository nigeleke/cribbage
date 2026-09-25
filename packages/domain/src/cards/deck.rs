use crate::constants::*;

use crate::{Card, Hand, Hands};

use super::pile::Pile;

/// Marker type for the main deck (the draw pile).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeckType;

/// The deck contains all cards at the start of the game and is drawn from
/// during dealing and for the starter cut.
pub type Deck = Pile<DeckType>;

impl Deck {
    pub fn new() -> Self {
        let cards = Card::all();
        Deck::from(cards)
    }

    pub fn cut(&mut self) -> Card {
        self.pop().expect("available card")
    }

    pub fn deal(&mut self) -> Hands {
        let hands = std::array::from_fn(|_| {
            let cards = self.take(CARDS_DEALT_PER_HAND);
            Hand::from(cards)
        });
        Hands::from(hands)
    }
}

#[cfg(test)]
mod test {
    use crate::Player;

    use super::*;

    #[test]
    fn contains_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.len(), STANDARD_DECK_SIZE);
    }

    #[test]
    fn contains_all_cards_for_all_suits_and_faces() {
        let deck = Deck::new();
        let cards = Card::all();
        cards.iter().for_each(|c| assert!(deck.contains(c)));
    }

    #[test]
    fn allow_a_random_card_to_be_cut() {
        let deck0 = Deck::new();
        let mut deck1 = deck0.clone();
        let cut = deck1.cut();

        assert!(deck0.contains(&cut));
        assert!(!deck1.contains(&cut));
        assert_eq!(deck1.len(), 51);
    }

    #[test]
    fn allow_deals() {
        let deck0 = Deck::new();
        let mut deck1 = deck0.clone();
        let deals = deck1.deal();
        assert!(deck0.contains_all(&deals[Player::Player0]));
        assert!(deck0.contains_all(&deals[Player::Player1]));
        assert!(deck1.contains_none(&deals[Player::Player0]));
        assert!(deck1.contains_none(&deals[Player::Player1]));
    }
}
