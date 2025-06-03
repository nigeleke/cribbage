#[cfg(feature = "server")]
use crate::database::UnstartedGameRow;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnstartedGameId(Uuid);

impl UnstartedGameId {
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UnstartedGameId {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        Self(uuid)
    }
}

impl From<Uuid> for UnstartedGameId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl FromStr for UnstartedGameId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(s).map_err(|e| e.to_string())?;
        Ok(Self(uuid))
    }
}

impl std::fmt::Display for UnstartedGameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnstartedGame {
    id: UnstartedGameId,
    name: String,
}

impl UnstartedGame {
    pub fn id(&self) -> &UnstartedGameId {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(feature = "server")]
impl From<UnstartedGameRow> for UnstartedGame {
    fn from(value: UnstartedGameRow) -> Self {
        Self {
            id: UnstartedGameId::from(value.id),
            name: value.name,
        }
    }
}

impl std::fmt::Display for UnstartedGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}
