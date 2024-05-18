use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Player(Uuid);

impl Player {
    pub(crate) fn new() -> Self { Self::default() }
}

impl Default for Player {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
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