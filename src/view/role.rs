use crate::types::prelude::Player;

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Role {
    CurrentPlayer,
    Opponent
} 

impl Role {
    pub fn opponent(&self) -> Role {
        match self {
            Role::CurrentPlayer => Role::Opponent,
            Role::Opponent => Role::CurrentPlayer,
        }
    }
}

pub type Dealer = Role;

impl From<(Player, Player)> for Dealer {
    fn from((a_player, the_player): (Player, Player)) -> Self {
        if a_player == the_player {
            Role::CurrentPlayer
        } else {
            Role::Opponent
        }
    }
}
