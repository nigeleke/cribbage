#[cfg(feature = "server")]
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::ServerStateExtractor;
use crate::{
    dto::{CardIdDTO, GameIdDTO, UserIdDTO},
    error::ApiError,
};

#[post("/api/{user_id}/game/{game_id}/play/{card}", State(server_state): State<ServerStateExtractor>)]
pub async fn play_card(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
    card: CardIdDTO,
) -> Result<(), ApiError> {
    use std::str::FromStr;

    use server::{
        action::play_card,
        domain::{Card, GameId, UserId},
    };

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());
    let card = Card::from_str(&card).map_err(ServerFnError::new)?;

    play_card(server_state.0, user_id, game_id, card).await?;
    Ok(())
}
