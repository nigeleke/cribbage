use crate::domain::GameId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumString)]
pub enum Availability {
    Private,
    Public,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailableGame {
    id: GameId,
    name: String,
    availability: Availability,
    created_at: DateTime<Utc>,
}

impl AvailableGame {
    pub fn new(
        id: GameId,
        name: String,
        availability: Availability,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            availability,
            created_at,
        }
    }

    pub fn id(&self) -> &GameId {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn availability(&self) -> &Availability {
        &self.availability
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
