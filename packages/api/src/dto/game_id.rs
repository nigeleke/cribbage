use serde::{Deserialize, Serialize};
// use server::domain::GameId;
use uuid::Uuid;

use crate::dto::DTOError;

/// Data transfer object for a game identifier, wrapping a ULID for unique, sortable identification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct GameIdDTO(Uuid);

impl GameIdDTO {
    /// Internal value of the id.
    pub fn value(self) -> Uuid {
        self.0
    }
}

impl Default for GameIdDTO {
    fn default() -> Self {
        Self(Uuid::nil())
    }
}

impl std::fmt::Display for GameIdDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for GameIdDTO {
    type Err = DTOError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(s)?;
        Ok(GameIdDTO(uuid))
    }
}

impl From<Uuid> for GameIdDTO {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
