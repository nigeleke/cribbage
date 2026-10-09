use cribbage_domain::prelude::Player;
use serde::{Deserialize, Serialize};

use crate::user::UserId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Users {
    host: UserId,
    guest: UserId,
}

impl Users {
    pub fn new(host: UserId, guest: UserId) -> Self {
        Self { host, guest }
    }

    pub fn player(&self, user: UserId) -> Option<Player> {
        match user {
            user if user == self.host => Some(Player::Player0),
            user if user == self.guest => Some(Player::Player1),
            _ => None,
        }
    }

    pub fn user(&self, player: Player) -> UserId {
        match player {
            Player::Player0 => self.host,
            Player::Player1 => self.guest,
        }
    }
}
