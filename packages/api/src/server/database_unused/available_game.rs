use serde::{Deserialize, Serialize};

use crate::GameIdDTO;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailableGame {
    Lobby { id: GameIdDTO, name: String },
    Active { id: GameIdDTO, name: String },
}

impl AvailableGame {
    pub fn id(&self) -> &GameIdDTO {
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

#[cfg(feature = "server")]
impl From<crate::AvailableGameRow> for AvailableGame {
    fn from(value: crate::AvailableGameRow) -> Self {
        if value.source == "Active" {
            Self::Active {
                id: GameId::from(value.id),
                name: value.name,
            }
        } else {
            Self::Lobby {
                id: GameId::from(value.id),
                name: value.name,
            }
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
