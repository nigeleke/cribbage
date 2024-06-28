use super::breakdown::Reasons;
use super::pegging::{Pegging, Peggings};

use crate::fmt::format_hashmap;
use crate::types::{Player, Players, HasPoints};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Scores(Peggings, Reasons);

impl Scores {
    pub fn new(players: &Players) -> Self {
        let peggings = Peggings::from_iter(players.iter().map(|&p| (p, Pegging::default())));
        let reasons = Reasons::default();
        Self(peggings, reasons)
    }

    pub fn peggings(&self) -> Peggings {
        self.0.clone()
    }

    pub fn reasons(&self) -> Reasons {
        self.1.clone()
    }

    pub fn add(&mut self, player: Player, reasons: &Reasons) {
        let points = reasons.points();
        let peggings = &mut self.0;
        peggings.insert(player, peggings[&player].add(points));
        self.1 += reasons.clone();
    }
}

impl std::fmt::Display for Scores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Peggings({}) Reasons({})", format_hashmap(&self.0), self.1)
    }
}

#[cfg(test)]
impl From<&Peggings> for Scores {
    fn from(value: &Peggings) -> Self {
        Self(value.clone(), Reasons::default())
    }
}
