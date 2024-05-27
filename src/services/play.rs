use crate::view::prelude::{Card, Game as GameView};

use leptos::*;

#[server]
pub async fn play(game_id: String, card: Card) -> Result<GameView, ServerFnError> {
    use crate::domain::prelude::*;
    use crate::ssr::auth;
    use crate::ssr::database::prelude::*;
    use crate::ssr::opponent::Opponent;
    use uuid::Uuid;

    let player: Player = auth::authenticated_user().await?.into();
    let game_id = Uuid::try_parse(&game_id)?;

    let connection = get_database().await;
    connection.acquire().await?;

    let mut transaction = connection.begin().await?;

    let game = select_game(&mut transaction, &game_id).await?;
    // TODO: let game = game.play(&player, &card)?;

    let opponent = game.opponent(&player);
    let game = Opponent::discard(&opponent, &game);

    update_game(&mut transaction, &game_id, &game).await?;

    transaction
        .commit()
        .await
        .map_err(ServerFnError::WrappedServerError)?;

    Ok(GameView::from((game, player)))
}
