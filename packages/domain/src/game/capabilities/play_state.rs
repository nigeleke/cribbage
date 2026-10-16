use crate::game::{Card, Game, Playing};
use crate::players::Player;
use crate::plays::PlayState;

/// Provides read-only access to the players' hands.
pub trait HasPlayState {
    /// Returns the current play state.
    fn play_state(&self) -> &PlayState;

    /// Returns the next play to play or go.
    fn next_to_play(&self) -> Player {
        self.play_state().next_to_play()
    }

    /// Returns true if all cards have been played.
    fn is_finished(&self) -> bool {
        self.play_state().is_finished()
    }

    /// Returns legal plays for player.
    fn legal_plays(&self, player: Player) -> Vec<Card> {
        self.play_state().legal_plays(player)
    }
}

impl HasPlayState for Playing {
    fn play_state(&self) -> &PlayState {
        &self.play_state
    }
}

impl<T: HasPlayState> HasPlayState for Game<T> {
    fn play_state(&self) -> &PlayState {
        self.state.play_state()
    }
}
