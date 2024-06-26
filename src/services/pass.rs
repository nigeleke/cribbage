use crate::view::Game as GameView;

use leptos::*;

#[server]
pub async fn pass(game_id: String) -> Result<GameView, ServerFnError> {
    use crate::domain::*;
    use crate::ssr::auth;
    use crate::ssr::database::*;
    use crate::ssr::opponent::Opponent;
    use crate::types::Player;
    use uuid::Uuid;

    let player: Player = auth::authenticated_user().await?.into();
    let game_id = Uuid::try_parse(&game_id)?;

    let connection = get_database().await;
    connection.acquire().await?;

    let mut transaction = connection.begin().await?;

    let mut game = select_game(&mut transaction, &game_id).await?;
    game = game.pass(player)?;

    if let Game::Playing(_, _, _, _, _, _) = game {
        let opponent = game.opponent(player);
        game = Opponent::play(opponent, &game);
    }

    update_game(&mut transaction, &game_id, &game).await?;

    transaction
        .commit()
        .await
        .map_err(ServerFnError::WrappedServerError)?;

    Ok(GameView::from((game, player)))
}
