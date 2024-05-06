use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Player(Uuid);

impl Player {
    pub fn new() -> Self { Player(Uuid::new_v4()) }
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
