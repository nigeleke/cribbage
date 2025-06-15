use super::GameId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailableGame {
    Lobby { id: GameId, name: String },
    Active { id: GameId, name: String },
}

impl AvailableGame {
    pub fn id(&self) -> &GameId {
        match self {
            Self::Lobby { id, name: _ } | Self::Active { id, name: _ } => id,
        }
    }

    pub fn name(&self) -> &String {
        match self {
            Self::Lobby { id: _, name } | Self::Active { id: _, name } => name,
        }
    }
}

// impl From<ActiveGame> for AvailableGame {
//     fn from(value: ActiveGame) -> Self {
//         Self::Active {
//             id: *value.id(),
//             name: value.name().clone(),
//         }
//     }
// }

// impl std::fmt::Display for AvailableGame {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.name().fmt(f)
//     }
// }
