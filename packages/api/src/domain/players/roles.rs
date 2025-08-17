use super::{Dealer, Pone};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Roles {
    dealer: Dealer,
    pone: Pone,
}

impl Roles {
    pub const fn new(dealer: Dealer, pone: Pone) -> Self {
        Self { dealer, pone }
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
        write!(f, "{dealer}, {pone}")
    }
}
