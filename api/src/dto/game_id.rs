use super::DtoError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameId(Uuid);

impl GameId {
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl Default for GameId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for GameId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl FromStr for GameId {
    type Err = DtoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::from_str(s)?;
        Ok(Self(id))
    }
}

impl std::fmt::Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
