use serde::{Deserialize, Serialize};

use super::{Dealer, Pone};
use crate::domain::{CutsForDeal, PLAYER0, PLAYER1, constants::PLAYER_COUNT};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Roles {
    dealer: Dealer,
    pone: Pone,
}

pub trait HasRoles {
    fn roles(&self) -> &Roles;
    fn roles_mut(&mut self) -> &mut Roles;

    fn dealer(&self) -> &Dealer {
        &self.roles().dealer
    }

    fn pone(&self) -> &Pone {
        &self.roles().pone
    }
}

impl Roles {
    pub fn new(dealer: Dealer) -> Self {
        Self {
            dealer,
            pone: dealer.opponent(),
        }
    }

    pub fn from_cuts(cuts: &CutsForDeal) -> Option<Self> {
        use std::cmp::Ordering;

        let defined_cuts = cuts.iter().filter_map(|c| *c).collect::<Vec<_>>();

        (defined_cuts.len() == PLAYER_COUNT)
            .then(|| {
                let dealer = match defined_cuts[PLAYER0]
                    .face()
                    .rank()
                    .cmp(&defined_cuts[PLAYER1].face().rank())
                {
                    Ordering::Less => Some(Dealer::from(PLAYER0)),
                    Ordering::Greater => Some(Dealer::from(PLAYER1)),
                    Ordering::Equal => None,
                };

                dealer.map(Self::new)
            })
            .flatten()
    }

    pub fn dealer(&self) -> &Dealer {
        &self.dealer
    }

    pub fn pone(&self) -> &Pone {
        &self.pone
    }

    pub fn swap(&mut self) {
        let was_dealer = self.dealer.player();
        let was_pone = self.pone.player();
        self.dealer = Dealer::from(was_pone);
        self.pone = Pone::from(was_dealer);
    }
}

impl std::fmt::Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { dealer, pone } = self;
        write!(f, "{dealer}, {pone}",)
    }
}
