use crate::{Card, Player};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Play {
    player: Player,
    card: Card,
}

impl Play {
    pub fn new<P: Into<Player>>(player: P, card: Card) -> Self {
        let player = player.into();
        Self { player, card }
    }

    pub const fn player(self) -> Player {
        self.player
    }

    pub const fn card(self) -> Card {
        self.card
    }
}

impl std::fmt::Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.player, self.card)
    }
}
