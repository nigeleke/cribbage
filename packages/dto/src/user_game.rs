use serde::{Deserialize, Serialize};

use crate::CardDTO;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    User,
    Opponent,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    #[default]
    Lobby,
    CutForDeal {
        user_cut: Option<CardDTO>,
        opponent_cut: Option<CardDTO>,
    },
    Active {
        dealer: Player,
        crib: Vec<CardDTO>,
    },
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
    phase: Phase,
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

    pub fn with_user_cut(self, user_cut: Option<CardDTO>) -> Self {
        let UserGameDTO {
            name,
            mut phase,
            user_state,
            opponent_state,
            cut,
            plays,
        } = self;
        phase = match phase {
            Phase::Lobby => Phase::CutForDeal {
                user_cut,
                opponent_cut: None,
            },
            Phase::CutForDeal { opponent_cut, .. } => Phase::CutForDeal {
                user_cut,
                opponent_cut,
            },
            Phase::Active { .. } => unreachable!(),
        };
        UserGameDTO {
            name,
            phase,
            user_state,
            opponent_state,
            cut,
            plays,
        }
    }

    pub fn with_opponent_cut(self, opponent_cut: Option<CardDTO>) -> Self {
        let UserGameDTO {
            name,
            mut phase,
            user_state,
            opponent_state,
            cut,
            plays,
        } = self;
        phase = match phase {
            Phase::Lobby => Phase::CutForDeal {
                user_cut: None,
                opponent_cut,
            },
            Phase::CutForDeal { user_cut, .. } => Phase::CutForDeal {
                user_cut,
                opponent_cut,
            },
            Phase::Active { .. } => unreachable!(),
        };
        UserGameDTO {
            name,
            phase,
            user_state,
            opponent_state,
            cut,
            plays,
        }
    }

    pub fn phase(&self) -> &Phase {
        &self.phase
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
