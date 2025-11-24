use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::DTOError;

/// Data transfer object for a game identifier, wrapping a ULID for unique, sortable identification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameIdDTO(Uuid);

impl GameIdDTO {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn value(self) -> Uuid {
        self.0
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
        let uuid = Uuid::from_str(s)?; //.map_err(|e| ApiError::InvalidUlid(e.to_string()))?;
        Ok(GameIdDTO(uuid))
    }
}

impl From<Uuid> for GameIdDTO {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
