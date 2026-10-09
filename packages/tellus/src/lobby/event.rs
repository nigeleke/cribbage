use serde::{Deserialize, Serialize};
use tellus::{SchemaVersion, Versioned};

use crate::persistence::PersistedCards;
use crate::user::UserId;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LobbyEvent {
    GameCreated { host: UserId, deck: PersistedCards },
    GuestJoined { guest: UserId },
}

impl Versioned for LobbyEvent {
    const MANIFEST: &'static str = "cribbage.lobby-event";
    const VERSION: SchemaVersion = SchemaVersion::new(1);
}
