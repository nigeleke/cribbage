use crate::domain::prelude::Player as DomainPlayer;

use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Role {
    CurrentPlayer,
    Opponent
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
