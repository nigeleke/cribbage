use crate::domain::{Hand, Player, constants::*};

/// Fixed-size array holding all players' hands.
///
/// Indexed by host & guest.
pub type Hands = [Hand; PLAYER_COUNT];

/// Trait for game states that have player hands.
pub trait HasHands {
    /// Immutable access to all hands.
    fn hands(&self) -> &Hands;

    /// Mutable access to all hands.
    fn hands_mut(&mut self) -> &mut Hands;

    /// Returns an immutable reference to the given player's hand.
    #[must_use]
    #[inline(always)]
    fn hand(&self, player: Player) -> &Hand {
        &self.hands()[player]
    }

    /// Returns a mutable reference to the given player's hand.
    #[must_use]
    #[inline(always)]
    fn hand_mut(&mut self, player: Player) -> &mut Hand {
        &mut self.hands_mut()[player]
    }
}
