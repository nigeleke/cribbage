use leptos::*;

#[server]
pub async fn create_game() -> Result<(), ServerFnError> {
    use crate::domain::*;
    use crate::ssr::auth;
    use crate::ssr::database::*;
    use crate::types::{Player, Players};

    let user = auth::authenticated_user().await?;
    let opponent = Player::new();
    let players = Players::from_iter(vec![user.into(), opponent].into_iter());
    
    let game = Game::new(&players)?;

    let connection = get_database().await;
    connection.acquire().await?;

    let mut transaction = connection.begin().await?;
    let game_id = insert_game(&mut transaction, &game).await?;

    transaction
        .commit()
        .await
        .map_err(ServerFnError::WrappedServerError)?;

    let path = format!("game/{}", game_id.to_string());
    leptos_axum::redirect(&path);

    Ok(())
}
