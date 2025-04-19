use crate::display::format_hashmap;
use crate::domain::{Cut, Peggings, Player};

#[derive(Debug)]
pub struct Finished {
    winner: Player,
    peggings: Peggings,
    cut: Cut,
}

impl Finished {
    pub fn new(winner: Player, peggings: Peggings, cut: Cut) -> Self {
        Self {
            winner,
            peggings,
            cut,
        }
    }

    pub fn winner(&self) -> Player {
        self.winner
    }

    pub fn peggings(&self) -> &Peggings {
        &self.peggings
    }

    pub fn cut(&self) -> Cut {
        self.cut
    }
}

impl std::fmt::Display for Finished {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Finished(winner: {}, peggings: {}, cut: {})",
            self.winner,
            format_hashmap(&self.peggings),
            self.cut
        )
    }
}
