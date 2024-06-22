use crate::domain::Card;
use crate::fmt::{format_hashmap, format_vec};
use crate::types::*;

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Play {
    player: Player,
    card: Card,
}

impl Play {
    pub fn new(player: Player, card: Card) -> Self {
        Self { player, card }
    }

    pub fn player(self) -> Player {
        self.player
    }

    pub fn card(self) -> Card {
        self.card
    }

    pub fn value(self) -> Value {
        self.card.value()
    }

    pub fn rank(self) -> Rank {
        self.card.rank()
    }
}

impl std::fmt::Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} -> {})", self.player, self.card)
    }
}
