use dioxus::prelude::*;

use crate::cqrs::Command;
use crate::database::{UpdateGame, append_events, select_game, update_game};
use crate::domain::{CutForDeal, CutForDealState, Event, GameId, UserId};
use crate::error::BackendError;
use crate::server_state::SERVER_STATE;
use crate::services::convertors;

pub async fn cut_for_deal(
    user_id: UserId,
    game_id: GameId,
) -> Result<CutForDealState, BackendError> {
    let id = game_id.value();
    let game = select_game(SERVER_STATE.postgres_pool(), id).await?;
    let game = game
        .map(convertors::game_row_to_game)
        .transpose()?
        .ok_or(BackendError::GameNotFound(game_id))?;

    let player = game.user_to_player(user_id)?;

    let command = CutForDeal::new(player);
    let (events, updated_game) = command.execute(game).await?;

    let state: JsonValue = convertors::state_to_json(updated_game.state());

    let update = UpdateGame {
        id,
        state: Some(state),
        ..Default::default()
    };

    let cut = events
        .iter()
        .find_map(|e| matches!(e, Event::CutForDealMade { cut, .. }).map(cut));

    let events = events
        .iter()
        .map(convertors::event_to_json)
        .collect::<Vec<_>>();

    let cut = updated_game.cut();

    let mut tx = SERVER_STATE.postgres_pool().begin().await?;
    let _ = update_game(tx.as_mut(), &new_game).await?;
    let _ = append_events(tx.as_mut(), id, events).await?;
    let _ = tx.commit().await?;

    Ok(cut)
}
