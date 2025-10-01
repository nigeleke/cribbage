use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data transfer object for a user identifier, wrapping a UUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserIdDTO(Uuid);

impl UserIdDTO {
    pub fn value(self) -> Uuid {
        self.0
    }
}

impl Default for UserIdDTO {
    fn default() -> Self {
        Self(Uuid::nil())
    }
}
