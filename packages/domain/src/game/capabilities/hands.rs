use crate::game::{
    Cutting, Discarding, Finished, Game, Hand, Playing, ScoringCrib, ScoringDealer, ScoringPone,
};
use crate::players::Player;
use crate::types::Hands;

/// Provides read-only access to the players' hands.
pub trait HasHands {
    /// Returns the hands of both players.
    fn hands(&self) -> &Hands;

    /// Returns the hand belonging to `player`.
    fn hand(&self, player: Player) -> &Hand {
        &self.hands()[player]
    }
}

impl HasHands for Discarding {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl HasHands for Cutting {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl HasHands for Playing {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl HasHands for ScoringPone {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl HasHands for ScoringDealer {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl HasHands for ScoringCrib {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl HasHands for Finished {
    fn hands(&self) -> &Hands {
        &self.hands
    }
}

impl<T: HasHands> HasHands for Game<T> {
    /// Return the hands in this current game.
    fn hands(&self) -> &Hands {
        self.state.hands()
    }
}
