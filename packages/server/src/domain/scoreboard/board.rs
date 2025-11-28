use serde::{Deserialize, Serialize};

use crate::{
    display::format_vec,
    domain::{PLAYER0, PLAYER1, Pegging, Peggings, Player, Points, ScoreBreakdown, constants::*},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scoreboard {
    peggings: Peggings,
    history: Vec<ScoreBreakdown>,
}

pub trait HasScoreboard {
    fn scoreboard(&self) -> &Scoreboard;
    fn scoreboard_mut(&mut self) -> &mut Scoreboard;

    fn pegging(&self, player: Player) -> &Pegging {
        &self.scoreboard().peggings[player]
    }
}

impl Scoreboard {
    pub fn new(peggings: Peggings) -> Self {
        Self {
            peggings,
            history: Vec::default(),
        }
    }

    pub fn pegging(&self, player: Player) -> &Pegging {
        &self.peggings[player]
    }

    pub fn peg(&mut self, player: Player, breakdown: &ScoreBreakdown) -> Option<Player> {
        let points = breakdown.points();
        if points > Points::from(0) {
            self.peggings[player] += points;
        }

        (self.peggings[player].points() >= Points::from(WINNING_SCORE)).then_some(player)
    }

    pub fn winner(&self) -> Option<Player> {
        if self.pegging(PLAYER0).points() >= Points::from(WINNING_SCORE) {
            Some(PLAYER0)
        } else if self.pegging(PLAYER1).points() >= Points::from(WINNING_SCORE) {
            Some(PLAYER1)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Scoreboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { peggings, .. } = self;
        let peggings = format_vec(peggings);
        write!(f, "Peggings({peggings})")
    }
}
