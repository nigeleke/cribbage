use crate::{Card, Player};

/// Represents a single play in the pegging phase of the game.
///
/// A `Play` pairs a player with the card they played.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Play {
    player: Player,
    card: Card,
}

impl Play {
    /// Creates a new `Play` with the given player and card.
    pub fn new(player: Player, card: Card) -> Self {
        Self { player, card }
    }

    /// Returns the player who made this play.
    pub const fn player(&self) -> Player {
        self.player
    }

    /// Returns the card played.
    pub const fn card(&self) -> Card {
        self.card
    }
}

impl std::fmt::Debug for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.player, self.card)
    }
}

pub fn plays_to_string(plays: &[Play]) -> String {
    plays
        .iter()
        .map(|p| format!("{:?}", p))
        .collect::<Vec<_>>()
        .join(", ")
}
