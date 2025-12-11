use std::collections::HashSet;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::domain::{PLAYER0, PLAYER1, Player};

/// Represents players who are to acknowledge the current game phase before
/// the game can proceed to the next phase.
///
/// `Pending` is used to implement synchronous turn progression:
/// a phase (e.g. bidding, playing a card, revealing tricks) only advances
/// when **all** expected players have called `acknowledge()`.
///
/// An empty set means the phase is complete and the game may proceed.
///
/// Note: there is now `new` as the `default` pending state will be pending
/// for both players in the game.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Pending(HashSet<Player>);

/// Trait for game states that track player acknowledgments.
pub trait HasPending {
    /// Immutable access to the pending set.
    fn pending(&self) -> &Pending;

    /// Mutable access to the pending set.
    fn pending_mut(&mut self) -> &mut Pending;
}

impl Pending {
    /// Returns `true` if all players have acknowledged (i.e. phase is complete).
    #[must_use]
    #[inline]
    pub fn finished(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the given player still needs to acknowledge.
    #[must_use]
    #[inline]
    pub fn waiting_on(&self, player: Player) -> bool {
        self.0.contains(&player)
    }

    /// Registers acknowledgment from a player.
    ///
    /// Returns `true` if both players have acknowledged and play can continue.
    #[must_use]
    pub fn acknowledge(&mut self, player: Player) -> bool {
        self.0.remove(&player);
        self.0.is_empty()
    }
}

impl Default for Pending {
    fn default() -> Self {
        Self(HashSet::from([PLAYER0, PLAYER1]))
    }
}

#[cfg(test)]
impl From<&[Player]> for Pending {
    fn from(value: &[Player]) -> Self {
        Self(HashSet::from_iter(value.to_owned()))
    }
}

impl std::fmt::Display for Pending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pending = self.0.iter().map(|p| p.to_string()).join(", ");
        write!(f, "Pending({pending})")
    }
}
