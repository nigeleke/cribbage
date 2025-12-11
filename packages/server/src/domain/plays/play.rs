use serde::{Deserialize, Serialize};

use crate::domain::{Card, Player};

/// Represents a single play in the pegging phase of the game.
///
/// A `Play` pairs a player with the card they played.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Play {
    player: Player,
    card: Card,
}

impl Play {
    /// Creates a new `Play` with the given player and card.
    #[must_use]
    pub fn new<P: Into<Player>>(player: P, card: Card) -> Self {
        let player = player.into();
        Self { player, card }
    }

    /// Returns the player who made this play.
    #[must_use]
    pub const fn player(&self) -> Player {
        self.player
    }

    /// Returns the card played.
    #[must_use]
    pub const fn card(&self) -> Card {
        self.card
    }
}

impl std::fmt::Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.player, self.card)
    }
}
