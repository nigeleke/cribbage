use serde::{Deserialize, Serialize};
use tellus::{SchemaVersion, Versioned};

use crate::user::UserId;

/// An event recording a change to a lobby.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LobbyEvent {
    /// Records a host in the lobby (lobby ).
    HostEntered {
        /// The user who created the session.
        host: UserId,
    },

    /// Records a guest joining the hosted session.
    GuestJoined {
        /// The user who joined as the guest.
        guest: UserId,
    },
}

impl Versioned for LobbyEvent {
    const MANIFEST: &'static str = "cribbage.lobby-event";
    const VERSION: SchemaVersion = SchemaVersion::new(1);
}
