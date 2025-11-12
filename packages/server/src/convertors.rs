use std::str::FromStr;

use serde_json::json;
use thiserror::*;

use crate::database::{AvailableGameRow, GameRow};
use crate::domain::{AvailableGame, AvailableGameSource, Game, GameId, State, UserId};

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error(transparent)]
    ParseError(#[from] strum::ParseError),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

pub fn available_game_row_to_available_game(
    row: AvailableGameRow,
) -> Result<AvailableGame, ConversionError> {
    let id = GameId::from(row.id);
    let name = row.name;
    let user = UserId::from(row.user_id);
    let source = AvailableGameSource::from_str(&row.source)?;

    let game = AvailableGame::new(id, user, name, source);

    Ok(game)
}

pub fn game_row_to_game(row: GameRow) -> Result<Game, ConversionError> {
    let id = GameId::from(row.id);
    let name = row.name;
    let host = UserId::from(row.host_id);
    let guest = row.guest_id.map(UserId::from);
    let state = json_to_state(row.state)?;

    let game = Game::new(id, host, guest, &name, state);

    Ok(game)
}

pub fn game_to_available_game(game: &Game, user: &UserId) -> AvailableGame {
    let id = game.id();
    let name = game.name().clone();
    let source = if game.guest().is_none() {
        AvailableGameSource::Lobby
    } else {
        AvailableGameSource::Active
    };
    AvailableGame::new(*id, *user, name, source)
}

#[inline]
pub fn json_to_state(json: serde_json::Value) -> Result<State, ConversionError> {
    let state = serde_json::from_value::<State>(json)?;
    Ok(state)
}

#[inline]
pub fn state_to_json(state: &State) -> serde_json::Value {
    json!(state).into()
}

// pub fn json_to_game_event(json: serde_json::Value) -> Result<GameEvent, ConversionError> {
//     let event = serde_json::from_value::<GameEvent>(json)?;
//     Ok(event)
// }

// pub fn game_event_to_json(event: &GameEvent) -> serde_json::Value {
//     json!(event).into()
// }
