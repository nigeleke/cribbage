use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};

use crate::{GameId, UserId};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, AsRefStr, EnumString)]
pub enum Source {
    Lobby,
    Active,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailableGame {
    id: GameId,
    user: UserId,
    name: String,
    source: Source,
}

impl AvailableGame {
    pub fn new(id: GameId, user: UserId, name: String, source: Source) -> Self {
        Self {
            id,
            user,
            name,
            source,
        }
    }

    pub fn id(&self) -> &GameId {
        &self.id
    }

    pub fn user(&self) -> &UserId {
        &self.user
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn source(&self) -> &Source {
        &self.source
    }
}
