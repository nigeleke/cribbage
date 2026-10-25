use serde::{Deserialize, Serialize};
use tellus::{SchemaVersion, Versioned};

use crate::game::GameId;
use crate::user::UserId;

/// The current state of a lobby and its hosted session.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LobbyState {
    /// The lobby has no hosted session.
    Unpaired,

    /// A host is waiting for a guest to join.
    WaitingForGuest {
        /// The user hosting the session.
        host: UserId,
    },

    /// A host and guest are paired in the lobby.
    Paired {
        /// The user hosting the session.
        host: UserId,
        /// The user who joined as the guest.
        guest: UserId,
        /// The identifier of the hosted session.
        game: GameId,
    },
}

impl Versioned for LobbyState {
    /// The identifier of the game-state schema.
    const MANIFEST: &'static str = "cribbage.lobby-state";
    /// The version of the game-state schema.
    const VERSION: SchemaVersion = SchemaVersion::new(1);
}
