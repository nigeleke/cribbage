use cribbage_macros::IdV4;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identity for a Lobby.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, IdV4)]
pub struct LobbyId(Uuid);
