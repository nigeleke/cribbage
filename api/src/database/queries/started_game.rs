use crate::database::{DatabaseError, StartedGameRow, TableChangeEvent};
use async_stream::stream;
use chrono::{DateTime, Utc};
use dioxus::logger::tracing::warn;
use futures::Stream;
use sqlx::{Executor, PgPool, Postgres, Result, postgres::PgListener};
use uuid::Uuid;

pub async fn insert_started_game<'e, E>(
    exec: E,
    game: &StartedGameRow,
) -> Result<StartedGameRow, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        INSERT INTO started_games (unstarted_game_id, active_game_id, created_at)
        VALUES ($1, $2, $3)
        RETURNING unstarted_game_id, active_game_id, created_at;
    "#;

    let game = sqlx::query_as::<_, StartedGameRow>(query)
        .bind(game.unstarted_game_id)
        .bind(game.active_game_id)
        .bind(game.created_at)
        .fetch_one(exec)
        .await?;

    Ok(game)
}

pub async fn delete_started_game<'e, E>(exec: E, game_id: Uuid) -> Result<(), DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        DELETE FROM started_games
        WHERE unstarted_game_id = $1;
    "#;

    sqlx::query(query).bind(game_id).execute(exec).await?;

    Ok(())
}

pub async fn listen_started_games_changes(
    pool: &PgPool,
) -> Result<
    impl Stream<Item = Result<TableChangeEvent<StartedGameRow>, DatabaseError>>,
    DatabaseError,
> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen("started_games_change").await?;

    Ok(stream! {
        while let Some(notification) = listener.try_recv().await? {
            match serde_json::from_str::<TableChangeEvent<StartedGameRow>>(notification.payload()) {
                Ok(change) => yield Ok(change),
                Err(e) => warn!("Failed to deserialize started_game event: {}", e.to_string()),
            }
        }
    })
}
