use crate::{CutsForDeal, Dealer, Player, Pone};

/// Represents the roles assigned to players in a round.
///
/// This struct captures which player is the dealer and which is the pone
/// (non-dealer) in the game.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Roles {
    dealer: Dealer,
    pone: Pone,
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

    /// Returns the dealer.
    pub fn dealer(self) -> Dealer {
        self.dealer
    }

    /// Returns the pone.
    pub fn pone(self) -> Pone {
        self.pone
    }

    /// Swaps the dealer and pone roles in place.
    pub fn swap(&mut self) {
        let was_dealer = self.dealer.player();
        let was_pone = self.pone.player();
        self.dealer = Dealer::from(was_pone);
        self.pone = Pone::from(was_dealer);
    }
}

/// Constructs roles, if possible, from a set of cuts.
///
/// Returns `None` if the roles cannot be determined (e.g., cuts are equal or incomplete).
impl TryFrom<&CutsForDeal> for Roles {
    type Error = ();

    fn try_from(value: &CutsForDeal) -> Result<Self, Self::Error> {
        let cut0 = value[Player::Player0].ok_or(())?;
        let cut1 = value[Player::Player1].ok_or(())?;

        match cut0.face().cmp(&cut1.face()) {
            std::cmp::Ordering::Less => Ok(Roles::new(Dealer::from(Player::Player0))),
            std::cmp::Ordering::Equal => Err(()),
            std::cmp::Ordering::Greater => Ok(Roles::new(Dealer::from(Player::Player1))),
        }
    }
}

impl std::fmt::Debug for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "roles({:?}, {:?})", self.dealer, self.pone)
    }
}

#[cfg(test)]
mod tests {
    use crate::Player;

    use macros::*;

    use super::*;

    #[test]
    fn roles_assign_pone_as_dealers_opponent() {
        let roles = Roles::new(Dealer::from(Player::Player0));

        assert_eq!(roles.dealer(), Dealer::from(Player::Player0));
        assert_eq!(roles.pone(), Pone::from(Player::Player1));
    }

    #[test]
    fn roles_assign_dealer_as_pones_opponent() {
        let roles = Roles::new(Dealer::from(Player::Player1));

        assert_eq!(roles.dealer(), Dealer::from(Player::Player1));
        assert_eq!(roles.pone(), Pone::from(Player::Player0));
    }

    #[test]
    fn roles_can_swap() {
        let mut roles = Roles::new(Dealer::from(Player::Player0));

        roles.swap();

        assert_eq!(roles.dealer(), Dealer::from(Player::Player1));
        assert_eq!(roles.pone(), Pone::from(Player::Player0));
    }

    #[test]
    fn swapping_roles_twice_restores_original_roles() {
        let mut roles = Roles::new(Dealer::from(Player::Player0));

        roles.swap();
        roles.swap();

        assert_eq!(roles.dealer(), Dealer::from(Player::Player0));
        assert_eq!(roles.pone(), Pone::from(Player::Player1));
    }

    #[test]
    fn roles_from_cuts_selects_player0_when_player0_cuts_lower_card() {
        let cuts = CutsForDeal::from([Some(card!("2H")), Some(card!("3H"))]);
        let roles = Roles::try_from(&cuts).expect("roles should be created from cuts");

        assert_eq!(roles.dealer(), Dealer::from(Player::Player0));
        assert_eq!(roles.pone(), Pone::from(Player::Player1));
    }

    #[test]
    fn roles_from_cuts_selects_player1_when_player1_cuts_lower_card() {
        let cuts = CutsForDeal::from([Some(card!("3H")), Some(card!("2H"))]);

        let roles = Roles::try_from(&cuts).expect("roles should be created from cuts");

        assert_eq!(roles.dealer(), Dealer::from(Player::Player1));
        assert_eq!(roles.pone(), Pone::from(Player::Player0));
    }

    #[test]
    fn roles_from_cuts_returns_none_when_cuts_are_equal() {
        let cuts = CutsForDeal::from([Some(card!("3H")), Some(card!("3S"))]);
        assert_eq!(Roles::try_from(&cuts), Err(()));
    }

    #[test]
    fn roles_from_cuts_returns_none_when_player0_cut_is_missing() {
        let cuts = CutsForDeal::from([None, Some(card!("3H"))]);
        assert_eq!(Roles::try_from(&cuts), Err(()));
    }

    #[test]
    fn roles_from_cuts_returns_none_when_player1_cut_is_missing() {
        let cuts = CutsForDeal::from([Some(card!("3H")), None]);
        assert_eq!(Roles::try_from(&cuts), Err(()));
    }

    #[test]
    fn roles_from_cuts_ignores_suit() {
        let cuts = CutsForDeal::from([Some(card!("2H")), Some(card!("3S"))]);
        let roles = Roles::try_from(&cuts).expect("roles should be created from cuts");
        assert_eq!(roles.dealer(), Dealer::from(Player::Player0));
    }
}
