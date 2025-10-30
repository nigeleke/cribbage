use dioxus::prelude::*;

use crate::database::{UpdateGame, select_game, update_game};
use crate::domain::{CutForDealState, GameId, UserId};
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;
use crate::services::convertors;

pub async fn cut_for_deal(
    user_id: UserId,
    game_id: GameId,
) -> Result<CutForDealState, BackendError> {
    let game_row = select_game(SERVER_STATE.postgres_pool(), game_id.value()).await?;

    let game = game_row
        .map(convertors::game_row_to_game)
        .transpose()?
        .ok_or(BackendError::GameNotFound(game_id))?;

    let (cut_for_deal_state, game) = game.cut_for_deal(user_id)?;
    let id = game_id.value();
    let state = Some(convertors::state_to_json(game.state()));

    let new_game = UpdateGame {
        id,
        state,
        ..Default::default()
    };

    let _ = update_game(SERVER_STATE.postgres_pool(), &new_game).await?;

    Ok(cut_for_deal_state)
}
