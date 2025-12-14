use constants::*;
use serde::{Deserialize, Serialize};

use crate::{
    display::format_vec,
    domain::{PLAYER0, PLAYER1, Pegging, Player, Points, Position, Positions},
};

/// Represents the scoreboard.
///
/// Tracks each player's position and the history of score sheets.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scoreboard {
    positions: Positions,
    history: Vec<Pegging>,
}

/// Trait for types that expose a `Scoreboard`.
pub trait HasScoreboard {
    /// Returns an immutable reference to the scoreboard.
    fn scoreboard(&self) -> &Scoreboard;

    /// Returns a mutable reference to the scoreboard.
    fn scoreboard_mut(&mut self) -> &mut Scoreboard;
}

impl Scoreboard {
    /// Creates a new scoreboard with the given positions.
    pub fn new(positions: Positions) -> Self {
        Self {
            positions,
            history: Vec::default(),
        }
    }

    /// Returns the position of the specified player.
    #[must_use]
    pub fn position(&self, player: Player) -> Position {
        self.positions[player]
    }

    /// Updates the scoreboard with the given pegging.
    ///
    /// If the player reached the winning score then they will be returned
    /// as Some(winner) otherwise None is returned.
    #[must_use]
    pub fn peg(&mut self, pegging: &Pegging) -> Option<Player> {
        let player = pegging.recipient();
        let sheet = pegging.score_sheet();

        let points = sheet.points();
        if points > Points::from(0) {
            self.positions[player] += points;
            self.history.push(pegging.clone());
        }

        (self.positions[player].points() >= Points::from(WINNING_SCORE)).then_some(*player)
    }

    /// Returns a reference to the most recent pegging from the history
    /// if any pegging rounds have been completed.
    ///
    /// Returns `None` if no pegging has occurred yet.
    #[must_use]
    pub fn latest_pegging(&self) -> Option<&Pegging> {
        self.history.last()
    }

    /// Returns the winner if any player has reached the winning score.
    #[must_use]
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
