use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::collections::HashSet;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Player(Uuid);

pub type Players = HashSet<Player>;

impl Player {
    #[cfg(any(feature = "ssr", test))]
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}

impl Eq for Player { }

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
