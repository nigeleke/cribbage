use crate::domain::prelude::Player as DomainPlayer;

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

impl From<(DomainPlayer, DomainPlayer)> for Dealer {
    fn from((a_player, the_player): (DomainPlayer, DomainPlayer)) -> Self {
        if a_player == the_player {
            Role::CurrentPlayer
        } else {
            Role::Opponent
        }
    }
}
