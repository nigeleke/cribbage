use cribbage_domain::prelude::Deck;

/// Provides a new deck of cards for each round of a game.
pub trait DeckSource: Send + Sync {
    /// Creates a new deck of cards.
    fn new_deck(&self) -> Deck;
}
