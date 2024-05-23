use super::player::Player;
use super::card::Card;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Play {
    player: Player,
    card: Card,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayState {
    next_to_play: Player,
    pass_count: usize,
    current_plays: Vec<Play>,
    previous_plays: Vec<Play>,
}

impl PlayState {
    pub(crate) fn new(next_to_play: Player) -> Self {
        Self { next_to_play, pass_count: 0, current_plays: vec![], previous_plays: vec![] }
    }

    #[cfg(test)]
    pub(crate) fn next_to_play(&self) -> Player {
        self.next_to_play
    }

    #[cfg(test)]
    pub(crate) fn pass_count(&self) -> usize {
        self.pass_count
    }

    #[cfg(test)]
    pub(crate) fn current_plays(&self) -> Vec<Play> {
        self.current_plays.clone()
    }

    #[cfg(test)]
    pub(crate) fn previous_plays(&self) -> Vec<Play> {
        self.previous_plays.clone()
    }
}