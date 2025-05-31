use super::{
    breakdown::Reasons,
    pegging::{Pegging, Peggings},
};
use crate::{
    constants::WINNING_SCORE,
    display::format_hashmap,
    domain::{Player, Players},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scores {
    peggings: Peggings,
    reasons: Reasons,
}

impl Scores {
    pub fn new(players: &Players) -> Self {
        let peggings = Peggings::from_iter(players.iter().map(|&p| (p, Pegging::default())));
        let reasons = Reasons::default();
        Self { peggings, reasons }
    }

    pub fn score_points(&mut self, player: Player, reasons: &Reasons) {
        let points = reasons.points();
        let peggings = &mut self.peggings;
        peggings.insert(player, peggings[&player].add(points));
        self.reasons += reasons.clone();
    }

    pub fn winner(&self) -> Option<Player> {
        self.peggings
            .iter()
            .filter_map(|(player, score)| (*score.points() >= WINNING_SCORE).then_some(*player))
            .next()
    }

    pub const fn peggings(&self) -> &Peggings {
        &self.peggings
    }

    pub fn pegging(&self, player: Player) -> &Pegging {
        &self.peggings[&player]
    }
}

pub trait HasScores {
    fn scores(&self) -> &Scores;

    #[cfg(test)]
    fn peggings(&self) -> &Peggings {
        self.scores().peggings()
    }

    fn pegging(&self, player: Player) -> &Pegging {
        self.scores().pegging(player)
    }
}

impl std::fmt::Display for Scores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Peggings({}) Reasons({})",
            format_hashmap(&self.peggings),
            self.reasons
        )
    }
}

impl From<&Peggings> for Scores {
    fn from(value: &Peggings) -> Self {
        Self {
            peggings: value.clone(),
            reasons: Reasons::default(),
        }
    }
}
