use crate::view::prelude::{Card, Game as GameView};

use leptos::*;

#[server]
pub async fn discard(game_id: String, cards: Vec<Card>) -> Result<GameView, ServerFnError> {
    use crate::ssr::auth;
    use crate::ssr::database::prelude::*;
    use crate::ssr::opponent::Opponent;
    use crate::types::prelude::Player;
    use uuid::Uuid;

    let player: Player = auth::authenticated_user().await?.into();
    let game_id = Uuid::try_parse(&game_id)?;

    let connection = get_database().await;
    connection.acquire().await?;

    let mut transaction = connection.begin().await?;

    let game = select_game(&mut transaction, &game_id).await?;
    let game = game.discard(player, &cards)?;

    let opponent = game.opponent(player);
    let mut game = Opponent::discard(opponent, &game);

    if game.dealer() == player {
        game = Opponent::play(opponent, &game);
    }

    update_game(&mut transaction, &game_id, &game).await?;

    transaction
        .commit()
        .await
        .map_err(ServerFnError::WrappedServerError)?;

    Ok(GameView::from((game, player)))
}
