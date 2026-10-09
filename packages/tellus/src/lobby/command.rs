use crate::persistence::PersistedCards;
use crate::user::UserId;

#[derive(Debug)]
pub enum LobbyCommand {
    Create { host: UserId, deck: PersistedCards },
    Join { guest: UserId },
}
