use super::card::{Card, CardSlot};
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
    card: CardSlot,
}

impl Play {
    pub(crate) fn card(&self) -> CardSlot {
        self.card
    }
}

impl From<(DomainPlay, DomainPlayer)> for Play {
    fn from((play, player): (DomainPlay, DomainPlayer)) -> Self {
        let role = (play.player(), player).into();
        let card = CardSlot::FaceUp(play.card());
        Play { role, card }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayState {
    running_total: usize,
    finished_plays: bool,
    legal_plays: Vec<Card>,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

impl PlayState {
    pub(crate) fn finished_plays(&self) -> bool {
        self.finished_plays
    }

    pub(crate) fn must_pass(&self) -> bool {
        self.legal_plays.is_empty()
    }

    pub(crate) fn legal_plays(&self) -> DomainHand {
        DomainHand::from(self.legal_plays.clone())
    }

    pub(crate) fn running_total(&self) -> usize {
        self.running_total
    }

    pub(crate) fn current_plays(&self) -> Vec<Play> {
        self.current_plays.clone()
    }

    pub(crate) fn previous_plays(&self) -> Vec<Play> {
        self.previous_plays.clone()
    }
}

impl From<(DomainPlayState, DomainPlayer)> for PlayState {
    fn from((play_state, player): (DomainPlayState, DomainPlayer)) -> Self {
        
        let running_total = play_state.running_total().into();

        let finished_plays = play_state.next_to_play().is_none();

        let legal_plays = if !finished_plays {
            play_state.legal_plays(player).ok().unwrap().cards()
        } else {
            vec![]
        };
        
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

        PlayState { running_total, finished_plays, legal_plays, current_plays, previous_plays }
    }
}

