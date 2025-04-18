use crate::display::format_hashmap;
use crate::domain::{Peggings, Player};

#[derive(Debug)]
pub struct Finished {
    winner: Player,
    peggings: Peggings,
}

impl Finished {
    pub fn new(winner: Player, peggings: Peggings) -> Self {
        Self { winner, peggings }
    }

    pub fn winner(&self) -> Player {
        self.winner
    }

    pub fn peggings(&self) -> &Peggings {
        &self.peggings
    }
}

impl std::fmt::Display for Finished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Finished(winner: {}, peggings: {})",
            self.winner,
            format_hashmap(&self.peggings)
        )
    }
}
