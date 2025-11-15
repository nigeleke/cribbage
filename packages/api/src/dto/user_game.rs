use serde::{Deserialize, Serialize};

use super::{CardDTO, PlayerDTO, PlayerStateDTO, PlaysDTO, ScoreDTO};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    #[default]
    Lobby,
    CutForDeal {
        user_cut: Option<CardDTO>,
        opponent_cut: Option<CardDTO>,
        dealer: Option<PlayerDTO>,
    },
    Active {
        dealer: PlayerDTO,
        crib: Vec<CardDTO>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserGameDTO {
    pub name: String,
    pub phase: Phase,
    pub pending: bool,
    pub user_state: PlayerStateDTO,
    pub opponent_state: PlayerStateDTO,
    pub cut: Option<CardDTO>,
    pub plays: Option<PlaysDTO>,
    pub winner: Option<PlayerDTO>,
}

impl UserGameDTO {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }

    pub fn with_user_cut(self, user_cut: Option<CardDTO>, dealer: Option<PlayerDTO>) -> Self {
        let Self {
            name,
            mut phase,
            pending: _,
            user_state,
            opponent_state,
            cut,
            plays,
            winner,
        } = self;

        let pending = dealer.is_none();

        phase = match phase {
            Phase::Lobby => Phase::CutForDeal {
                user_cut,
                opponent_cut: None,
                dealer,
            },
            Phase::CutForDeal { opponent_cut, .. } => Phase::CutForDeal {
                user_cut,
                opponent_cut,
                dealer,
            },
            Phase::Active { .. } => {
                unreachable!()
            }
        };

        Self {
            name,
            phase,
            pending,
            user_state,
            opponent_state,
            cut,
            plays,
            winner,
        }
    }

    pub fn with_opponent_cut(self, opponent_cut: Option<CardDTO>) -> Self {
        let Self {
            name,
            mut phase,
            pending,
            user_state,
            opponent_state,
            cut,
            plays,
            winner,
        } = self;

        phase = match phase {
            Phase::Lobby => Phase::CutForDeal {
                user_cut: None,
                opponent_cut,
                dealer: None,
            },
            Phase::CutForDeal {
                user_cut, dealer, ..
            } => Phase::CutForDeal {
                user_cut,
                opponent_cut,
                dealer,
            },
            Phase::Active { .. } => {
                unreachable!()
            }
        };

        Self {
            name,
            phase,
            pending,
            user_state,
            opponent_state,
            cut,
            plays,
            winner,
        }
    }

    pub fn with_dealer_and_crib(mut self, dealer: PlayerDTO, crib: &[CardDTO]) -> Self {
        let crib = Vec::from(crib);
        self.phase = Phase::Active { dealer, crib };
        self
    }

    pub fn with_user_state(mut self, score: ScoreDTO, hand: &[CardDTO]) -> Self {
        let hand = Vec::from(hand);
        self.user_state = PlayerStateDTO { hand, score };
        self
    }

    pub fn with_opponent_state(mut self, score: ScoreDTO, hand: &[CardDTO]) -> Self {
        let hand = Vec::from(hand);
        self.opponent_state = PlayerStateDTO { hand, score };
        self
    }

    pub fn dealer(&self) -> Option<&PlayerDTO> {
        match &self.phase {
            Phase::Lobby => None,
            Phase::CutForDeal { dealer, .. } => dealer.as_ref(),
            Phase::Active { dealer, .. } => Some(dealer),
        }
    }
}
