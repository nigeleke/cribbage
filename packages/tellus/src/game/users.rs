use cribbage_domain::prelude::Player;
use serde::{Deserialize, Serialize};

use crate::user::UserId;

/// The two users participating in a cribbage game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Users {
    host: UserId,
    guest: UserId,
}

impl Users {
    /// Creates a game with the specified host and guest.
    pub fn new(host: UserId, guest: UserId) -> Self {
        Self { host, guest }
    }

    /// Returns the player associated with a user, or `None` if the user is not in the game.
    pub fn player(&self, user: UserId) -> Option<Player> {
        match user {
            user if user == self.host => Some(Player::Player0),
            user if user == self.guest => Some(Player::Player1),
            _ => None,
        }
    }

    /// Returns the user associated with a player.
    pub fn user(&self, player: Player) -> UserId {
        match player {
            Player::Player0 => self.host,
            Player::Player1 => self.guest,
        }
    }
}

