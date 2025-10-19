use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Player(usize);

pub const PLAYER0: Player = Player(0);
pub const PLAYER1: Player = Player(1);
pub const PLAYERS: [Player; 2] = [PLAYER0, PLAYER1];

impl Player {
    /// Returns the opponent of this player.
    pub const fn opponent(&self) -> Self {
        Self(1 - self.0)
    }
}

impl<T> std::ops::Index<Player> for [T] {
    type Output = T;

    fn index(&self, index: Player) -> &Self::Output {
        &self[index.0]
    }
}

impl<T> std::ops::Index<&Player> for [T] {
    type Output = T;

    fn index(&self, index: &Player) -> &Self::Output {
        &self[index.0]
    }
}

impl<T> std::ops::IndexMut<Player> for [T] {
    fn index_mut(&mut self, index: Player) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl<T> std::ops::IndexMut<&Player> for [T] {
    fn index_mut(&mut self, index: &Player) -> &mut Self::Output {
        &mut self[index.0]
    }
}

#[cfg(test)]
impl From<usize> for Player {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player({})", self.0)
    }
}
