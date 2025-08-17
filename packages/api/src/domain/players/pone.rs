use super::{Dealer, Player};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pone(Player);

impl Pone {
    pub const fn player(&self) -> Player {
        self.0
    }

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
