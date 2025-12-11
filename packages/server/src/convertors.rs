use std::str::FromStr;

use crate::{
    database::{AvailableGameRow, GameQueryRow},
    domain::{Availability, AvailableGame, Game, GameId},
    error::{ServerError, bug},
};

pub fn available_game_row_to_available_game(
    row: AvailableGameRow,
) -> Result<AvailableGame, ServerError> {
    let id = GameId::from(row.id);
    let name = row.name;
    let availability = Availability::from_str(&row.availability).map_err(bug!())?;

    let game = AvailableGame::new(id, name, availability);

    Ok(game)
}

#[inline]
pub fn game_query_row_to_game(row: GameQueryRow) -> Result<Game, ServerError> {
    let instance_value = row
        .payload
        .get("instance")
        .cloned()
        .ok_or_else(|| ServerError::Internal(anyhow::anyhow!("missing field 'instance'")))?;

    let game = serde_json::from_value::<Game>(instance_value).map_err(bug!())?;
    Ok(game)
}
