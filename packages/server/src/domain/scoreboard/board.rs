use serde::{Deserialize, Serialize};

use crate::{
    display::format_vec,
    domain::{
        PLAYER0, PLAYER1, Pegging, Player, Points, Position, Positions, ScoreSheet, constants::*,
    },
};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scoreboard {
    positions: Positions,
    history: Vec<ScoreSheet>,
}

pub trait HasScoreboard {
    fn scoreboard(&self) -> &Scoreboard;
    fn scoreboard_mut(&mut self) -> &mut Scoreboard;

    fn positions(&self, player: Player) -> &Position {
        &self.scoreboard().positions[player]
    }
}

impl Scoreboard {
    pub fn new(positions: Positions) -> Self {
        Self {
            positions,
            history: Vec::default(),
        }
    }

    pub fn position(&self, player: Player) -> &Position {
        &self.positions[player]
    }

    pub fn peg(&mut self, pegging: &Pegging) -> Option<Player> {
        let player = pegging.player();
        let sheet = pegging.score_sheet();

        let points = sheet.points();
        if points > Points::from(0) {
            self.positions[player] += points;
        }

        (self.positions[player].points() >= Points::from(WINNING_SCORE)).then_some(*player)
    }

    pub fn winner(&self) -> Option<Player> {
        if self.position(PLAYER0).points() >= Points::from(WINNING_SCORE) {
            Some(PLAYER0)
        } else if self.position(PLAYER1).points() >= Points::from(WINNING_SCORE) {
            Some(PLAYER1)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Scoreboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { positions, .. } = self;
        let positions = format_vec(positions);
        write!(f, "Scoreboard({positions})")
    }
}
