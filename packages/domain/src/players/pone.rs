use crate::players::player::Player;

/// Represents the pone (non-dealer) player in a two-player game.
///
/// This type wraps a `Player` and provides convenient access to the
/// corresponding pone in the round.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Pone(Player);

impl Pone {
    /// Returns the underlying `Player` representing the pone.
    pub(crate) const fn player(self) -> Player {
        self.0
    }
}

impl From<Player> for Pone {
    fn from(value: Player) -> Self {
        Self(value)
    }
}

impl std::fmt::Debug for Pone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pone({:?})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::players::player::Player;

    use super::*;

    #[test]
    fn pone_has_correct_player() {
        let pone = Pone::from(Player::Player0);
        assert_eq!(pone.player(), Player::Player0);
    }
}
