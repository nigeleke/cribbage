use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::domain::GameId;

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
}

impl AvailableGame {
    pub fn new(id: GameId, name: String, availability: Availability) -> Self {
        Self {
            id,
            name,
            availability,
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
}
