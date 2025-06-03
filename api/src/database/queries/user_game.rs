use crate::{
    DatabaseError,
    database::{TableChangeEvent, model::UserGameRow},
};
use async_stream::stream;
use dioxus::logger::tracing::warn;
use futures_util::Stream;
use sqlx::{Executor, PgPool, Postgres, Result, postgres::PgListener, types::JsonValue};
use uuid::Uuid;

pub async fn select_user_game<'e, E>(
    exec: E,
    game_id: &Uuid,
    user_id: &Uuid,
) -> Result<UserGameRow, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        SELECT game_id, user_id, state, created_at, updated_at
        FROM user_games
        WHERE game_id = $1 AND user_id = $2;
    "#;

    let game: UserGameRow = sqlx::query_as::<_, UserGameRow>(query)
        .bind(game_id)
        .bind(user_id)
        .fetch_one(exec)
        .await?;

    Ok(game)
}

pub async fn update_user_game_state<'e, E>(
    exec: E,
    game_id: &Uuid,
    user_id: &Uuid,
    state: &JsonValue,
) -> Result<UserGameRow, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        UPDATE user_games
        SET state = $1
        WHERE game_id = $2 AND user_id = $3
        RETURNING game_id, user_id, state, created_at, updated_at;
    "#;

    let game: UserGameRow = sqlx::query_as::<_, UserGameRow>(query)
        .bind(state)
        .bind(game_id)
        .bind(user_id)
        .fetch_one(exec)
        .await?;

    Ok(game)
}

pub async fn listen_user_games_changes(
    pool: &PgPool,
) -> Result<impl Stream<Item = Result<TableChangeEvent<UserGameRow>, DatabaseError>>, DatabaseError>
{
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen("user_games_change").await?;

    Ok(stream! {
        while let Some(notification) = listener.try_recv().await? {
            match serde_json::from_str::<TableChangeEvent<UserGameRow>>(notification.payload()) {
                Ok(change) => yield Ok(change),
                Err(e) => warn!("Failed to deserialize user_game event: {}", e.to_string()),
            }
        }
    })
}
