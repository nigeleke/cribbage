use super::{ActiveGame, ActiveGameId, UnstartedGameId};
#[cfg(feature = "server")]
use crate::database::AvailableGameRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvailableGameId {
    Unstarted(UnstartedGameId),
    Active(ActiveGameId),
}

impl AvailableGameId {
    pub fn value(&self) -> &Uuid {
        match self {
            Self::Unstarted(id) => id.value(),
            Self::Active(id) => id.value(),
        }
    }
}

impl std::fmt::Display for AvailableGameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unstarted(id) => id.fmt(f),
            Self::Active(id) => id.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvailableGame {
    pub(crate) id: AvailableGameId,
    pub(crate) name: String,
}

impl AvailableGame {
    pub fn id(&self) -> &AvailableGameId {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl From<ActiveGame> for AvailableGame {
    fn from(value: ActiveGame) -> Self {
        Self {
            id: AvailableGameId::Active(*value.id()),
            name: value.name().clone(),
        }
    }
}

#[cfg(feature = "server")]
impl From<AvailableGameRow> for AvailableGame {
    fn from(value: AvailableGameRow) -> Self {
        Self {
            id: if value.source == "Active" {
                AvailableGameId::Active(ActiveGameId::from(value.id))
            } else {
                AvailableGameId::Unstarted(UnstartedGameId::from(value.id))
            },
            name: value.name,
        }
    }
}

impl std::fmt::Display for AvailableGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}
