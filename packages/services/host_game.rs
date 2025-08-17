use crate::domain::LobbyGameId;
use dioxus::prelude::*;

#[server]
pub async fn host_game() -> Result<LobbyGameId, ServerFnError> {
    Ok(LobbyGameId::new())
}
