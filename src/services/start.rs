use crate::view::Game as GameView;

use leptos::*;
use uuid::Uuid;

#[server]
pub async fn start(id: String) -> Result<GameView, ServerFnError> {
    use crate::domain::prelude::{Card, Game, Player};
    use crate::ssr::auth;
    use crate::ssr::database::prelude::*;

    let player: Player = auth::authenticated_user().await?.into();
    let game_id = Uuid::try_parse(&id)?;

    let connection = get_database().await;
    connection.acquire().await?;

    let mut transaction = connection.begin().await?;

    let game = select_game(&mut transaction, &game_id).await?;

    transaction
        .commit()
        .await
        .map_err(ServerFnError::WrappedServerError)?;

    let view: GameView = (game, player).into();

    Ok(view)
}
