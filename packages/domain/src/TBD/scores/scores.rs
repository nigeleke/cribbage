use crate::{Player, Points, Position, Positions, constants::*};

/// Represents the scoreboard.
///
/// Tracks each player's position and the history of score sheets.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Scores {
    history: Vec<Pegging>,
}

impl Scores {
    /// Returns the position of the specified player.
    pub fn position(&self, player: Player) -> Position {
        self.positions[player]
    }

    /// Updates the scoreboard with the given pegging.
    ///
    /// If the player reached the winning score then they will be returned
    /// as Some(winner) otherwise None is returned.
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
    pub fn latest_pegging(&self) -> Option<&Pegging> {
        self.history.last()
    }

    /// Returns the winner if any player has reached the winning score.
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
