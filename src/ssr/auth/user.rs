use crate::types::Player;

use uuid::Uuid;

#[derive(Clone)]
pub struct User(Uuid);

impl From<Uuid> for User {
    fn from(value: Uuid) -> Self {
        User(value)
    }
}

impl Into<Player> for User {
    fn into(self) -> Player {
        Player::from(self.0)
    }
}
