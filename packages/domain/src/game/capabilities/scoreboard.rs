use crate::game::{Game, Score, Scoreboard};

/// Provides read-only access to the players' scoreboard.
pub trait HasScoreboard {
    /// Returns the game's current scoreboard.
    fn scoreboard(&self) -> &Scoreboard;
}

impl<T> HasScoreboard for Game<T> {
    fn scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }
}

impl<T: HasScoreboard> Game<T> {
    /// Returns the latest (most recent) score on the scoreboard, if any..
    pub fn latest_score(&self) -> Option<&Score> {
        self.scoreboard().latest_score()
    }
}
