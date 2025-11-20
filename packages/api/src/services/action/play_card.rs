use crate::{CardIdDTO, GameIdDTO, UserIdDTO};
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[post("/api/{user_id}/game/{game_id}/play/{card}", State(server_state): State<server::ServerState>)]
pub async fn play_card(user_id: UserIdDTO, game_id: GameIdDTO, card: CardIdDTO) -> Result<()> {
    use server::action::play_card;
    use server::domain::{Card, GameId, UserId};
    use std::str::FromStr;

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());
    let card = Card::from_str(&card).map_err(ServerFnError::new)?;

    let _ = play_card(server_state, user_id, game_id, card).await?;
    Ok(())
}
