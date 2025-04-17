use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::collections::{HashSet, hash_set};
use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Player(Uuid);

#[derive(Debug, PartialEq)]
pub struct Players(HashSet<Player>);

impl Players {
    pub fn iter(&self) -> hash_set::Iter<Player> {
        self.0.iter()
    }

    pub fn contains(&self, player: &Player) -> bool {
        self.0.contains(player)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn players_1_2(&self) -> (Player, Player) {
        let players = Vec::from_iter(&self.0);
        (*players[0], *players[1])
    }

    pub fn opponent(&self, player: Player) -> Player {
        let (player1, player2) = self.players_1_2();
        if player == player1 { player2 } else { player1 }
    }
}

pub trait HasPlayers {
    fn players(&self) -> Players;
}

impl FromIterator<Player> for Players {
    fn from_iter<T: IntoIterator<Item = Player>>(iter: T) -> Self {
        Self(HashSet::from_iter(iter))
    }
}

impl IntoIterator for Players {
    type Item = Player;
    type IntoIter = std::collections::hash_set::IntoIter<Player>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Player {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new()
    }
}

impl Eq for Player {}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<Uuid> for Player {
    fn from(value: Uuid) -> Self {
        Player(value)
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:8.8}", self.0.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_from_uuid_to_player() {
        let uuid0 = Uuid::new_v4();
        let player = Player::from(uuid0);
        let Player(uuid1) = player;
        assert_eq!(uuid1, uuid0);
    }
}
