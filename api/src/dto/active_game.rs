use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveGameId(Uuid);

impl ActiveGameId {
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for ActiveGameId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl FromStr for ActiveGameId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(s).map_err(|e| e.to_string())?;
        Ok(Self(uuid))
    }
}

impl std::fmt::Display for ActiveGameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
