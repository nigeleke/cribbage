/// Errors that can occur when processing a lobby.
#[derive(Debug, thiserror::Error)]
pub enum LobbyError {
    /// The event is not valid for the current lobby state.
    #[error("lobby-error.unhandled-event")]
    UnhandledEvent,
}
