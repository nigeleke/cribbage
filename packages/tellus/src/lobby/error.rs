use crate::user::UserId;

/// Errors that can occur when processing a lobby.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum LobbyError {
    /// The host is invalid. The lobby is already hosted.
    #[error("lobby-error.invalid-host")]
    InvalidHost(UserId),

    /// The guest is invalid. The only invalid guest is the host.
    #[error("lobby-error.invalid-guest")]
    InvalidGuest(UserId),

    /// The lobby is full; joins are not permitted.
    #[error("lobby-error.lobby-full")]
    LobbyFull,

    /// The lobby is empty; joins are not permitted.
    #[error("lobby-error.lobby-empty")]
    LobbyEmpty,

    /// The event is not valid for the current lobby state.
    #[error("lobby-error.unhandled-event")]
    UnhandledEvent,
}
