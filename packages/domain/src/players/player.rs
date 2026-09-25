use strum::EnumIter;

/// Represents a player in the game.
///
/// This is a simple wrapper around an index to distinguish players
/// in a type-safe manner. Only two players are supported in this implementation
/// of the game.
#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Player {
    /// The first player.
    Player0,

    /// The second player.
    Player1,
}

/// The fixed Players taking part in the game.
pub const PLAYERS: [Player; 2] = [Player::Player0, Player::Player1];

impl Player {
    /// Returns the opponent of this player.
    pub const fn opponent(&self) -> Self {
        match self {
            Self::Player0 => Self::Player1,
            Self::Player1 => Self::Player0,
        }
    }

    /// Returns the ordinal for the player entry.
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Player::Player0 => "player-0",
            Player::Player1 => "player-1",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player0_opponent_is_player1() {
        assert_eq!(Player::Player0.opponent(), Player::Player1);
    }

    #[test]
    fn player1_opponent_is_player0() {
        assert_eq!(Player::Player1.opponent(), Player::Player0);
    }

    #[test]
    fn players_contains_both_players() {
        assert_eq!(PLAYERS, [Player::Player0, Player::Player1]);
    }
}
