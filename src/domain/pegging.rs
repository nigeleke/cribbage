use crate::constants::WINNING_SCORE;
use crate::types::prelude::{Player, Points, HasPoints};

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Peg(Points);

impl HasPoints for Peg {
    fn points(&self) -> Points {
        self.0
    }
}

impl std::fmt::Display for Peg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Pegging(Peg, Peg);

pub type Peggings = HashMap<Player, Pegging>;

impl Pegging {
    pub fn front_peg(&self) -> Peg { self.1 }
    pub fn back_peg(&self) -> Peg { self.0 }

    pub fn add(&self, points: Points) -> Self {
        if points == 0.into() {
            *self
        } else {
            Self ( self.front_peg(), Peg(self.front_peg().points() + points))
        }
    }

    pub fn is_winning_score(&self) -> bool {
        self.points() >= WINNING_SCORE.into()
    }
}

impl HasPoints for Pegging {
    fn points(&self) -> Points {
        self.front_peg().points()
    }
}

impl std::fmt::Display for Pegging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.0, self.1)
    }
}
