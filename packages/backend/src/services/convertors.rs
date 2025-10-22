use serde_json::json;

use crate::database::GameRow;
use crate::domain::{Game, GameId, State, UserId};
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

// TODO: Remove?
// pub fn json_to_game(json: serde_json::Value) -> Result<Game, BackendError> {
//     dioxus::logger::tracing::info!("json_to_game from `{json}`");
//     let game = serde_json::from_value::<Game>(json)?;
//     Ok(game)
// }

pub fn json_to_state(json: serde_json::Value) -> Result<State, BackendError> {
    let state = serde_json::from_value::<State>(json)?;
    Ok(state)
}

pub fn state_to_json(state: &State) -> serde_json::Value {
    json!(state).into()
}
