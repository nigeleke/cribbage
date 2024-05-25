use crate::domain::prelude::Player as DomainPlayer;

use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Role {
    CurrentPlayer,
    Opponent
} 

pub type Dealer = Role;

impl From<(DomainPlayer, DomainPlayer)> for Dealer {
    fn from((current, dealer): (DomainPlayer, DomainPlayer)) -> Self {
        if current == dealer {
            Role::CurrentPlayer
        } else {
            Role::Opponent
        }
    }
}
