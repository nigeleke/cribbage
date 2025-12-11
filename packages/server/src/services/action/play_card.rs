use crate::{
    domain::{Card, GameCommand, GameId, UserId},
    error::{ServerError, bug},
    server_state::ServerState,
    services::queries::get_game,
};

/// Plays a card for the user in the pegging phase of a game.
///
/// This function updates the game state by playing the specified card
/// from the user's hand. It validates that the action is allowed for
/// the given user and that the play is legal according to game rules.
///
/// # Parameters
///
/// - `server_state`: The shared server state containing the game and database.
/// - `user_id`: The ID of the user playing the card.
/// - `game_id`: The ID of the game in which the card is being played.
/// - `card`: The card to play.
///
/// # Returns
///
/// Returns `Ok(())` if the card was successfully played.
/// Returns a `ServerError` if the action is forbidden, the game is not found,
/// the play is illegal, or another internal error occurs.
///
/// # Example
///
/// ```no_run
/// let card_to_play = Card::new(Face::Five, Suit::Hearts);
/// play_card(server_state.clone(), user_id, game_id, card_to_play).await?;
/// ```
pub async fn play_card(
    server_state: ServerState,
    user_id: UserId,
    game_id: GameId,
    card: Card,
) -> Result<(), ServerError> {
    if let Some(game) = get_game(server_state.clone(), game_id).await? {
        let player = game
            .validate_user(user_id)
            .ok_or(ServerError::Forbidden("play".into()))?;

        let aggregate_id = game_id.value().to_string();

        let command = GameCommand::PlayCard { player, card };

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
