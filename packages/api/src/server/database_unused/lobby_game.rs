use super::GameId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LobbyGame {
    id: GameId,
    name: String,
}

impl LobbyGame {
    pub fn id(&self) -> &GameId {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(feature = "server")]
impl From<crate::LobbyGameRow> for LobbyGame {
    fn from(value: crate::LobbyGameRow) -> Self {
        Self {
            id: GameId::from(value.id),
            name: value.name,
        }
    }
}

impl std::fmt::Display for LobbyGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}
