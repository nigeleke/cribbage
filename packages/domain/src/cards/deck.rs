use crate::constants::CARDS_DEALT_PER_HAND;
use crate::{Card, Hand, Hands};

/// The deck contains all cards at the start of the game and is drawn from
/// during dealing and for the starter cut.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Deck(Vec<Card>);

impl Deck {
    /// Create a new deck, with all the cards. Cards will be ordered and require
    /// shuffling by the client.
    /// To get a shuffled Deck do:
    ///
    /// ```
    /// # use cribbage_domain::prelude::*;
    /// let mut cards = Deck::new().to_vec();
    /// // Shuffle `cards` using the client's preferred RNG.
    /// let deck = Deck::from_iter(cards);
    /// ```
    pub fn new() -> Self {
        Deck(Card::all())
    }

    pub(crate) fn remove(&mut self, card: Card) {
        self.0.retain(|c| c != &card);
    }

    pub(crate) fn cut(&mut self) -> Card {
        self.0.pop().expect("available card")
    }

    pub(crate) fn deal(&mut self) -> Hands {
        let hands = std::array::from_fn(|_| {
            let cards = self.0.drain(..CARDS_DEALT_PER_HAND);
            Hand::from_iter(cards)
        });
        Hands::from(hands)
    }
}

impl FromIterator<Card> for Deck {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl std::ops::Deref for Deck {
    type Target = [Card];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::STANDARD_DECK_SIZE;

    use super::*;

    #[test]
    fn contains_52_cards() {
        let deck = Deck::new();
        assert_eq!(deck.0.len(), STANDARD_DECK_SIZE);
    }

    #[test]
    fn contains_all_cards_for_all_suits_and_faces() {
        let deck = Deck::new();
        let cards = Card::all();
        cards.iter().for_each(|c| assert!(deck.0.contains(c)));
    }

    //     #[test]
    //     fn allow_a_random_card_to_be_cut() {
    //         let deck0 = Deck::new();
    //         let mut deck1 = deck0.clone();
    //         let cut = deck1.cut();
    //
    //         assert!(deck0.contains(&cut));
    //         assert!(!deck1.contains(&cut));
    //         assert_eq!(deck1.len(), 51);
    //     }

    // todo!()
    // #[test]
    // fn allow_deals() {
    //     let deck0 = Deck::new();
    //     let mut deck1 = deck0.clone();
    //     let deals = deck1.deal();
    //     assert!(deck0.contains_all(&deals[Player::Player0]));
    //     assert!(deck0.contains_all(&deals[Player::Player1]));
    //     assert!(deck1.contains_none(&deals[Player::Player0]));
    //     assert!(deck1.contains_none(&deals[Player::Player1]));
    // }
}
