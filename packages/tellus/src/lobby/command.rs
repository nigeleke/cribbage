use crate::user::UserId;

/// A command to change the occupants of a lobby.
#[derive(Debug)]
pub enum LobbyCommand {
    /// Create a hosted session in an empty lobby.
    Create {
        /// The user hosting the session.
        host: UserId,
    },

    /// Join a hosted session in a lobby with an available slot.
    Join {
        /// The user joining as the guest.
        guest: UserId,
    },
}

