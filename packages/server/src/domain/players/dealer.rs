use serde::{Deserialize, Serialize};

use super::{Player, Pone};

/// Represents the dealer player in a two-player game.
///
/// This type wraps a `Player` and provides convenient access to the
/// corresponding dealer in the round.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Dealer(Player);

impl Dealer {
    /// Returns the player who is the dealer.
    pub const fn player(&self) -> Player {
        self.0
    }

    /// Returns the opponent of the dealer, wrapped as the `Pone`.
    pub fn opponent(&self) -> Pone {
        Pone::from(self.0.opponent())
    }
}

impl<T> std::ops::Index<Dealer> for [T] {
    type Output = T;

    fn index(&self, index: Dealer) -> &Self::Output {
        &self[index.0]
    }
}

impl<T> std::ops::Index<&Dealer> for [T] {
    type Output = T;

    fn index(&self, index: &Dealer) -> &Self::Output {
        &self[index.0]
    }
}

impl From<Player> for Dealer {
    fn from(value: Player) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Dealer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dealer({})", self.0)
    }
}
