use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedUserId(Uuid);

impl AuthenticatedUserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}
