use cribbage_domain::prelude::Deck;

use cribbage_tellus::prelude::DeckSource;

pub struct ShuffledDeckSource {
    deck: Deck,
}

impl ShuffledDeckSource {
    pub(crate) fn new() -> Self {
        let deck = Deck::new();
        Self { deck }
    }
}

impl Default for ShuffledDeckSource {
    fn default() -> Self {
        Self::new()
    }
}

impl DeckSource for ShuffledDeckSource {
    fn new_deck(&self) -> Deck {
        self.deck.clone()
    }
}
