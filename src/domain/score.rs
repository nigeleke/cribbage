use super::player::Player;

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

pub(crate) type BackPeg = usize;

pub(crate) type FrontPeg = usize;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Score(BackPeg, FrontPeg);

impl Score {
    pub fn front_peg(&self) -> FrontPeg { self.1 }
    pub fn back_peg(&self) -> BackPeg { self.0 }

    pub fn add(&self, score: usize) -> Self {
        if score == 0 {
            *self
        } else {
            Self ( self.front_peg(), self.front_peg() + score)
        }
    }

    pub fn value(&self) -> usize {
        self.front_peg()
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.0, self.1)
    }
}

pub type Scores = HashMap<Player, Score>;
