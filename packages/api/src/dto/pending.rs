use serde::{Deserialize, Serialize};

/// Represents which player is currently pending an action in the game.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PendingDTO {
    /// No player is pending.
    #[default]
    Nobody,

    /// The user is pending an action; this is prioritised even
    /// if the opponent is also pending.
    User,

    /// The opponent is pending an action.
    Opponent,
}

#[cfg(feature = "server")]
mod server_only {
    use server::domain::{Pending, Player};

    use super::*;

    impl PendingDTO {
        /// Creates a `PendingDTO` from the server domain `Pending` state for the given player.
        ///
        /// # Parameters
        /// - `player`: The player for whom to determine pending status.
        /// - `pending`: The domain `Pending` object representing game state.
        ///
        /// # Returns
        /// - `PendingDTO::User` if the player is waiting.
        /// - `PendingDTO::Opponent` if the opponent is waiting.
        /// - `PendingDTO::Nobody` if neither is waiting.
        pub fn new(player: Player, pending: &Pending) -> Self {
            let opponent = player.opponent();
            match (pending.waiting_on(player), pending.waiting_on(opponent)) {
                (true, _) => PendingDTO::User,
                (false, true) => PendingDTO::Opponent,
                _ => PendingDTO::Nobody,
            }
        }
    }
}
