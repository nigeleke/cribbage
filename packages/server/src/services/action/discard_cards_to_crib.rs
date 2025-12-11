use crate::{
    domain::{Card, GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
    services::queries::get_game,
};

/// Discards a set of cards from a player's hand to the crib in a game.
///
/// This function updates the game state to move the specified cards
/// from the user's hand into the crib. It validates that the action
/// is allowed for the given user and game.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user performing the discard.
/// - `game_id`: The ID of the game in which the discard occurs.
/// - `cards`: The cards to discard into the crib.
///
/// # Returns
///
/// Returns `Ok(())` if the discard was successful. Returns a `ServerError` if
/// the action is forbidden, the game is not found, or another internal error occurs.
pub async fn discard_cards_to_crib(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
    cards: Vec<Card>,
) -> Result<(), ServerError> {
    if let Some(game) = get_game(server_state.clone(), game_id).await? {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("discard cards to crib".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::DiscardCards { player, cards };

        server_state
            .cqrs
            .execute(&aggregate_id, command)
            .await
            .map_err(bug!())?;

        Ok(())
    } else {
        Err(ServerError::NotFound)
    }
}
