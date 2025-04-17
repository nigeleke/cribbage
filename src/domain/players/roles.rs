use std::cmp::Ordering;

use super::player::Player;

use crate::domain::{Card, Cuts};

use thiserror::*;

#[derive(Debug, Error, PartialEq)]
pub enum RolesError {
    #[error("roles cannot be determined using cuts {0} and [1]")]
    Indeterminate(Card, Card),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Roles {
    dealer: Player,
    pone: Player,
}

impl Roles {
    pub fn new(dealer: Player, pone: Player) -> Self {
        Self { dealer, pone }
    }

    pub fn dealer(&self) -> Player {
        self.dealer
    }

    pub fn pone(&self) -> Player {
        self.pone
    }
}

impl TryFrom<&Cuts> for Roles {
    type Error = RolesError;

    fn try_from(value: &Cuts) -> std::result::Result<Self, Self::Error> {
        let mut players = value.keys();

        let mut get_cut = || {
            let player = players
                .next()
                .expect(stringify!(TryFrom<&Cuts> for Roles::try_from));
            let rank = value[player].rank();
            (player, rank)
        };

        let (player1, rank1) = get_cut();
        let (player2, rank2) = get_cut();

        match rank1.cmp(&rank2) {
            Ordering::Less => Ok(Self::new(*player1, *player2)),
            Ordering::Greater => Ok(Self::new(*player2, *player1)),
            Ordering::Equal => Err(RolesError::Indeterminate(value[player1], value[player2])),
        }
    }
}

pub trait HasRoles {
    fn roles(&self) -> &Roles;

    fn dealer(&self) -> Player {
        self.roles().dealer()
    }

    fn pone(&self) -> Player {
        self.roles().pone()
    }
}
