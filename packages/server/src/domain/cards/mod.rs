mod crib;
mod cuts_for_deal;
mod deck;
mod error;
mod hand;
mod hands;
mod pile;
mod starter_cut;

pub use {
    crib::{Crib, HasCrib},
    cuts_for_deal::{CutsForDeal, HasCutsForDeal},
    deck::{Deck, HasDeck},
    error::CardsError,
    hand::Hand,
    hands::{Hands, HasHands},
    starter_cut::{HasStarterCut, StarterCut},
};

#[cfg(test)]
mod macros;

#[cfg(test)]
pub use deck::STANDARD_DECK_SIZE;
