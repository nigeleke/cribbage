use super::Peg;

use crate::constants::WINNING_SCORE;
use crate::types::{Player, Points, HasPoints};

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Score(Peg, Peg);

pub type Scores = HashMap<Player, Score>;

impl Score {
    pub fn front_peg(&self) -> Peg { self.1 }
    pub fn back_peg(&self) -> Peg { self.0 }

    pub fn add(&self, points: Points) -> Self {
        if points == 0.into() {
            *self
        } else {
            Self ( self.front_peg(), Peg::new(self.front_peg().points() + points))
        }
    }
}

impl HasPoints for Score {
    fn points(&self) -> Points {
        self.front_peg().points()
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.0, self.1)
    }
}
