use super::Peg;

use crate::types::{Player, Points, HasPoints};

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

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
            Self ( self.front_peg(), Peg::new(self.front_peg().points() + points))
        }
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
