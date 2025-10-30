use serde::{Deserialize, Serialize};

use super::{Dealer, Pone};
use crate::domain::{Cut, Cuts, PLAYER0, PLAYER1, Player, WaitingForCuts};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Roles {
    dealer: Dealer,
    pone: Pone,
    cuts: [Cut; 2],
}

impl Roles {
    pub fn from_cuts_when_ready(cuts: &Cuts, pending: &WaitingForCuts) -> Option<Self> {
        use std::cmp::Ordering;

        pending
            .finished()
            .then(|| {
                let defined_cuts = cuts.iter().filter_map(|c| c.clone()).collect::<Vec<_>>();
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
                    cuts: [defined_cuts[0], defined_cuts[1]],
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

    pub fn cut(&self, player: Player) -> &Cut {
        &self.cuts[player]
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
        let Self { dealer, pone, cuts } = self;
        write!(f, "{dealer}, {pone} {cuts:?}",)
    }
}
