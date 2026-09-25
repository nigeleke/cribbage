use constants::*;

use crate::domain::{Hand, Player};

/// Trait for game states that have player hands.
pub trait HasHands {
    /// Immutable access to all hands.
    fn hands(&self) -> &Hands;

    /// Mutable access to all hands.
    fn hands_mut(&mut self) -> &mut Hands;

    /// Returns an immutable reference to the given player's hand.
    #[inline(always)]
    fn hand(&self, player: Player) -> &Hand {
        &self.hands()[player]
    }

    /// Returns a mutable reference to the given player's hand.
    #[inline(always)]
    fn hand_mut(&mut self, player: Player) -> &mut Hand {
        &mut self.hands_mut()[player]
    }
}
