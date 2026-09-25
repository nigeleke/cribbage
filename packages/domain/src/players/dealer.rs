use crate::{Player, Pone};

/// Represents the dealer player in a two-player game.
///
/// This type wraps a `Player` and provides convenient access to the
/// corresponding dealer in the round.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Dealer(Player);

impl Dealer {
    /// Returns the player who is the dealer.
    pub const fn player(self) -> Player {
        self.0
    }

    /// Returns the opponent of the dealer, wrapped as the `Pone`.
    pub fn opponent(self) -> Pone {
        Pone::from(self.0.opponent())
    }
}

impl From<Player> for Dealer {
    fn from(value: Player) -> Self {
        Self(value)
    }
}

impl std::fmt::Debug for Dealer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dealer({:?})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Player;

    use super::*;

    #[test]
    fn dealer_has_correct_player() {
        let dealer = Dealer::from(Player::Player0);
        assert_eq!(dealer.player(), Player::Player0);
    }

    #[test]
    fn dealer_opponent_is_pone() {
        let dealer = Dealer::from(Player::Player0);
        assert_eq!(dealer.opponent(), Pone::from(Player::Player1));
    }

    #[test]
    fn dealer_opponent_is_symmetric() {
        let dealer = Dealer::from(Player::Player0);
        assert_eq!(dealer.opponent().opponent(), dealer);
    }
}
