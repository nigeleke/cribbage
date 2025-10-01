use super::GameId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveGame {
    id: GameId,
    name: String,
}

impl ActiveGame {
    pub fn id(&self) -> &GameId {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(feature = "server")]
impl From<crate::ActiveGameRow> for ActiveGame {
    fn from(value: crate::ActiveGameRow) -> Self {
        Self {
            id: GameId::from(value.id),
            name: value.name,
        }
    }
}
