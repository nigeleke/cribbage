use serde::{Deserialize, Serialize};

use super::{Dealer, Player};

/// Represents the pone (non-dealer) player in a two-player game.
///
/// This type wraps a `Player` and provides convenient access to the
/// corresponding pone in the round.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Pone(Player);

impl Pone {
    /// Returns the underlying `Player` representing the pone.
    #[must_use]
    pub const fn player(&self) -> Player {
        self.0
    }

    /// Returns the opponent (dealer) of this pone.
    #[must_use]
    pub fn opponent(&self) -> Dealer {
        Dealer::from(self.0.opponent())
    }
}

impl<T> std::ops::Index<Pone> for [T] {
    type Output = T;

    fn index(&self, index: Pone) -> &Self::Output {
        &self[index.0]
    }
}

impl<T> std::ops::Index<&Pone> for [T] {
    type Output = T;

    fn index(&self, index: &Pone) -> &Self::Output {
        &self[index.0]
    }
}

impl From<Player> for Pone {
    fn from(value: Player) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Pone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pone({})", self.0)
    }
}
