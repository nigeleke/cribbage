use cribbage_macros::IdV4;
use serde::{Deserialize, Serialize};

/// Unique identity for a User entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, IdV4)]
pub struct UserId(uuid::Uuid);
