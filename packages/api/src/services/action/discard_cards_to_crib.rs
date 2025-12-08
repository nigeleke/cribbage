#[cfg(feature = "server")]
use dioxus::fullstack::extract::State;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::ServerStateExtractor;
use crate::{
    dto::{CardIdDTO, GameIdDTO, UserIdDTO},
    error::ApiError,
};

#[post("/api/{user_id}/game/{game_id}/discard_cards_to_crib", State(server_state): State<ServerStateExtractor>)]
pub async fn discard_cards_to_crib(
    user_id: UserIdDTO,
    game_id: GameIdDTO,
    cards: Vec<CardIdDTO>,
) -> Result<(), ApiError> {
    use std::str::FromStr;

    use server::{
        action::discard_cards_to_crib,
        domain::{Card, GameId, UserId},
    };

    let user_id = UserId::from(user_id.value());
    let game_id = GameId::from(game_id.value());
    let cards = cards
        .iter()
        .map(|cid| Card::from_str(cid))
        .collect::<Result<_, _>>()
        .map_err(|error| ApiError::BadRequest(error.to_string()))?;

    discard_cards_to_crib(server_state.0, user_id, game_id, cards).await?;
    Ok(())
}
