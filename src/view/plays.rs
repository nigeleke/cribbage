use super::card::{Card, CardSlot};
use super::role::Role;

use crate::domain::{
    Hand as DomainHand,
    Play as DomainPlay,
    PlayState as DomainPlayState,
};
use crate::types::{Player, Value};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Play {
    role: Role,
    card: CardSlot,
}

impl Play {
    pub fn card(&self) -> CardSlot {
        self.card
    }
}

impl From<(DomainPlay, Player)> for Play {
    fn from((play, player): (DomainPlay, Player)) -> Self {
        let role = (play.player(), player).into();
        let card = CardSlot::FaceUp(play.card());
        Play { role, card }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayState {
    running_total: Value,
    all_cards_are_played: bool,
    legal_plays: Vec<Card>,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

impl PlayState {
    pub fn all_cards_are_played(&self) -> bool {
        self.all_cards_are_played
    }

    pub fn must_pass(&self) -> bool {
        self.legal_plays.is_empty()
    }

    pub fn legal_plays(&self) -> DomainHand {
        DomainHand::from(self.legal_plays.clone())
    }

    pub fn running_total(&self) -> Value {
        self.running_total
    }

    pub fn current_plays(&self) -> Vec<Play> {
        self.current_plays.clone()
    }

    pub fn previous_plays(&self) -> Vec<Play> {
        self.previous_plays.clone()
    }
}

impl From<(DomainPlayState, Player)> for PlayState {
    fn from((play_state, player): (DomainPlayState, Player)) -> Self {
        
        let running_total = play_state.running_total();

        let all_cards_are_played = play_state.all_are_cards_played();

        let mut legal_plays = Vec::new();
        if !all_cards_are_played {
            legal_plays = Vec::from(play_state.legal_plays(player).ok().unwrap().as_ref());
        }
        
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

        PlayState { running_total, all_cards_are_played, legal_plays, current_plays, previous_plays }
    }
}

