use constants::*;
use serde::{Deserialize, Serialize};

use super::{Dealer, Pone};
use crate::domain::{CutsForDeal, PLAYER0, PLAYER1};

/// Represents the roles assigned to players in a round.
///
/// This struct captures which player is the dealer and which is the pone
/// (non-dealer) in the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Roles {
    dealer: Dealer,
    pone: Pone,
}

/// Trait for types that contain player roles.
pub trait HasRoles {
    /// Returns an immutable reference to the roles.
    fn roles(&self) -> &Roles;

    /// Returns a mutable reference to the roles.
    fn roles_mut(&mut self) -> &mut Roles;

    /// Returns an immutable reference to the dealer.
    #[must_use]
    fn dealer(&self) -> &Dealer {
        &self.roles().dealer
    }

    /// Returns an immutable reference to the pone (non-dealer).
    #[must_use]
    fn pone(&self) -> &Pone {
        &self.roles().pone
    }
}

impl Roles {
    /// Creates a new `Roles` assignment given the dealer.
    ///
    /// The pone is automatically inferred as the opponent of the dealer.
    pub fn new(dealer: Dealer) -> Self {
        Self {
            dealer,
            pone: dealer.opponent(),
        }
    }

    /// Constructs roles, if possible, from a set of cuts.
    ///
    /// Returns `None` if the roles cannot be determined (e.g., cuts are equal or incomplete).
    #[must_use]
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

    /// Returns an immutable reference to the dealer.
    #[must_use]
    pub fn dealer(&self) -> &Dealer {
        &self.dealer
    }

    /// Returns an immutable reference to the pone.
    #[must_use]
    pub fn pone(&self) -> &Pone {
        &self.pone
    }

    /// Swaps the dealer and pone roles in place.
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
