use dioxus::logger::tracing::field::debug;
use serde::{Deserialize, Serialize};
use serde_json::de;

use super::{Dealer, Pone};
use crate::{
    constants::PLAYER_COUNT,
    domain::{Cuts, PLAYER0, PLAYER1, WaitingForCuts},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Roles {
    dealer: Dealer,
    pone: Pone,
}

impl Roles {
    pub fn from_cuts(cuts: &Cuts) -> Option<Self> {
        use std::cmp::Ordering;

        let defined_cuts = cuts.iter().filter_map(|c| c.clone()).collect::<Vec<_>>();

        (defined_cuts.len() == PLAYER_COUNT)
            .then(|| {
                debug("Roles:from_cuts: {defined_cuts:?}");
                let dealer = match defined_cuts[PLAYER0]
                    .face()
                    .rank()
                    .cmp(&defined_cuts[PLAYER1].face().rank())
                {
                    Ordering::Less => Some(Dealer::from(PLAYER0)),
                    Ordering::Greater => Some(Dealer::from(PLAYER1)),
                    Ordering::Equal => None,
                };

                dealer.map(|dealer| Self {
                    dealer,
                    pone: dealer.opponent(),
                })
            })
            .flatten()
    }

    pub const fn dealer(&self) -> &Dealer {
        &self.dealer
    }

    pub const fn pone(&self) -> &Pone {
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
