use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use crate::CardDTO;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, AsRefStr)]
pub enum Player {
    User,
    Opponent,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string().fmt(f)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    #[default]
    Lobby,
    CutForDeal {
        user_cut: Option<CardDTO>,
        opponent_cut: Option<CardDTO>,
        dealer: Option<Player>,
    },
    Active {
        dealer: Player,
        user_cut: CardDTO,
        opponent_cut: CardDTO,
        crib: Vec<CardDTO>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Score {
    back_peg: usize,
    front_peg: usize,
}

impl Score {
    pub fn new(back_peg: usize, front_peg: usize) -> Self {
        Self {
            back_peg,
            front_peg,
        }
    }
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
    pending: bool,
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

    pub fn with_user_cut(self, user_cut: Option<CardDTO>, dealer: Option<Player>) -> Self {
        eprintln!("UserGameDTO::with_user_cut {user_cut:?} {dealer:?}");
        let Self {
            name,
            mut phase,
            pending: _,
            user_state,
            opponent_state,
            cut,
            plays,
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
                eprintln!("applying UserGameDTO::with_user_cut in unexpected state");
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
        }
    }

    pub fn with_opponent_cut(self, opponent_cut: Option<CardDTO>) -> Self {
        eprintln!("UserGameDTO::with_opponent_cut {opponent_cut:?}");
        let Self {
            name,
            mut phase,
            pending,
            user_state,
            opponent_state,
            cut,
            plays,
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
                eprintln!("applying UserGameDTO::with_opponent_cut in unexpected state");
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
        }
    }

    pub fn with_dealer_and_crib(
        mut self,
        user_cut: CardDTO,
        opponent_cut: CardDTO,
        dealer: Player,
        crib: &[CardDTO],
    ) -> Self {
        eprintln!(
            "UserGameDTO::with_dealer_and_crib {user_cut:?} {opponent_cut:?} {dealer:?} {crib:#?}"
        );
        let crib = Vec::from(crib);
        self.phase = Phase::Active {
            dealer,
            user_cut,
            opponent_cut,
            crib,
        };
        self
    }

    pub fn with_user_state(mut self, score: Score, hand: &[CardDTO]) -> Self {
        let hand = Vec::from(hand);
        self.user_state = PlayerState { hand, score };
        self
    }

    pub fn with_opponent_state(mut self, score: Score, hand: &[CardDTO]) -> Self {
        let hand = Vec::from(hand);
        self.opponent_state = PlayerState { hand, score };
        self
    }

    pub fn phase(&self) -> &Phase {
        &self.phase
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn user_cut(&self) -> Option<&CardDTO> {
        match &self.phase {
            Phase::Lobby => None,
            Phase::CutForDeal { user_cut, .. } => user_cut.as_ref(),
            Phase::Active { user_cut, .. } => Some(user_cut),
        }
    }

    pub fn opponent_cut(&self) -> Option<&CardDTO> {
        match &self.phase {
            Phase::Lobby => None,
            Phase::CutForDeal { opponent_cut, .. } => opponent_cut.as_ref(),
            Phase::Active { opponent_cut, .. } => Some(opponent_cut),
        }
    }

    pub fn dealer(&self) -> Option<&Player> {
        match &self.phase {
            Phase::Lobby => None,
            Phase::CutForDeal { dealer, .. } => dealer.as_ref(),
            Phase::Active { dealer, .. } => Some(dealer),
        }
    }
}
