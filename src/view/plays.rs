use super::card::Card;
use super::role::Role;

use crate::domain::prelude::{
    Hand as DomainHand,
    Play as DomainPlay,
    PlayState as DomainPlayState,
    Player as DomainPlayer,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Play {
    role: Role,
    card: Card,
}

impl From<(DomainPlay, DomainPlayer)> for Play {
    fn from((play, player): (DomainPlay, DomainPlayer)) -> Self {
        let role = (play.player(), player).into();
        let card = play.card();
        Play { role, card }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayState {
    running_total: usize,
    legal_plays: Vec<Card>,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

impl PlayState {
    pub(crate) fn must_pass(&self) -> bool {
        self.legal_plays.is_empty()
    }

    pub(crate) fn legal_plays(&self) -> DomainHand {
        DomainHand::from(self.legal_plays.clone())
    }
}

impl From<(DomainPlayState, DomainPlayer)> for PlayState {
    fn from((play_state, player): (DomainPlayState, DomainPlayer)) -> Self {
        let running_total = play_state.running_total().into();

        let legal_plays = play_state.legal_plays(player).ok().unwrap().cards();
        
        let current_plays = play_state
            .current_plays()
            .into_iter()
            .map(|p| (p, player).into())
            .collect::<Vec<_>>();
        
        let previous_plays = play_state
            .previous_plays()
            .into_iter()
            .map(|p| (p, player).into())
            .collect::<Vec<_>>();

        PlayState { running_total, legal_plays, current_plays, previous_plays }
    }
}

