use std::str::FromStr;

use serde_json::json;

use crate::database::{AvailableGameRow, GameRow};
use crate::domain::{AvailableGame, AvailableGameSource, Game, GameId, State, UserId};
use crate::error::BackendError;
use crate::services::convertors;

pub fn game_row_to_game(row: GameRow) -> Result<Game, BackendError> {
    let id = GameId::from(row.id);
    let name = row.name;
    let host = UserId::from(row.host_id);
    let guest = row.guest_id.map(UserId::from);
    let state = convertors::json_to_state(row.state)?;

    let game = Game::new(id, host, guest, name, state);

    Ok(game)
}

pub fn available_game_row_to_available_game(
    row: AvailableGameRow,
) -> Result<AvailableGame, BackendError> {
    let id = GameId::from(row.id);
    let name = row.name;
    let user = UserId::from(row.user_id);
    let source = AvailableGameSource::from_str(&row.source)?;

    let game = AvailableGame::new(id, user, name, source);

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

pub fn json_to_state(json: serde_json::Value) -> Result<State, BackendError> {
    let state = serde_json::from_value::<State>(json)?;
    Ok(state)
}

pub fn state_to_json(state: &State) -> serde_json::Value {
    json!(state).into()
}
