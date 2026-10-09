use cribbage_macros::IdV7;
use serde::{Deserialize, Serialize};

/// Unique identity for a Game entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, IdV7)]
pub struct GameId(uuid::Uuid);
