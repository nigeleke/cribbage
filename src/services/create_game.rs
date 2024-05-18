use leptos::*;

#[server]
pub async fn create_game() -> Result<(), ServerFnError> {
    use crate::domain::prelude::*;
    use crate::ssr::auth;
    use crate::ssr::database::prelude::*;
    use std::collections::HashSet;

    let user = auth::authenticated_user().await?;
    let opponent = Player::new();
    let players: HashSet<Player> = HashSet::from_iter(vec![user.into(), opponent].into_iter());
    
    let game = Game::new(&players)?;

    let connection = get_database().await;
    connection.acquire().await?;

    let mut transaction = connection.begin().await?;
    let game_id = insert_game(&mut transaction, &game).await?;

    transaction
        .commit()
        .await
        .map_err(ServerFnError::WrappedServerError)?;

    leptos_axum::redirect(&format!("game/{}", game_id.to_string()));

    Ok(())
}
