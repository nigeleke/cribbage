use serde::{Deserialize, Serialize};

use crate::CardDTO;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    User,
    Opponent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dealer {
    Undecided {
        user_cut: Option<CardDTO>,
        opponent_cut: Option<CardDTO>,
    },
    Decided {
        dealer: Player,
        crib: Vec<CardDTO>,
    },
}

impl Default for Dealer {
    fn default() -> Self {
        Self::Undecided {
            user_cut: None,
            opponent_cut: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct Score {
    back_peg: u8,
    front_peg: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct PlayerState {
    hand: Vec<CardDTO>,
    score: Score,
}

type Play = (Player, CardDTO);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Plays {
    current: Vec<Play>,
    historic: Vec<Play>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserGameDTO {
    name: String,
    dealer: Dealer,
    user_state: PlayerState,
    opponent_state: PlayerState,
    cut: Option<CardDTO>,
    plays: Option<Plays>,
}

impl UserGameDTO {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }

    pub fn dealer(&self) -> &Dealer {
        &self.dealer
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
