use crate::view::prelude::Game as GameView;

use leptos::*;

#[server]
pub async fn next_round(game_id: String) -> Result<GameView, ServerFnError> {
    use crate::domain::prelude::*;
    use crate::ssr::auth;
    use crate::ssr::database::prelude::*;
    use uuid::Uuid;

    let player: Player = auth::authenticated_user().await?.into();
    let game_id = Uuid::try_parse(&game_id)?;

    let connection = get_database().await;
    connection.acquire().await?;

    let mut transaction = connection.begin().await?;

    let mut game = select_game(&mut transaction, &game_id).await?;
    game = game.deal_next_hands()?;

    update_game(&mut transaction, &game_id, &game).await?;

    transaction
        .commit()
        .await
        .map_err(ServerFnError::WrappedServerError)?;

    Ok(GameView::from((game, player)))
}
