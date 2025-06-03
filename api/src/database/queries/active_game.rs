use crate::{
    DatabaseError,
    database::{TableChangeEvent, model::ActiveGameRow},
};
use async_stream::stream;
use dioxus::logger::tracing::warn;
use futures_util::Stream;
use sqlx::{Executor, PgPool, Postgres, Result, postgres::PgListener, types::JsonValue};
use uuid::Uuid;

pub async fn insert_active_game<'e, E>(exec: E, game: &ActiveGameRow) -> Result<Uuid, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        INSERT INTO active_games (id, name, user_id1, user_id2, state, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, name, user_id1, user_id2, state, created_at, updated_at;
    "#;

    let game: ActiveGameRow = sqlx::query_as::<_, ActiveGameRow>(query)
        .bind(game.id)
        .bind(&game.name)
        .bind(game.user_id1)
        .bind(game.user_id2)
        .bind(&game.state)
        .bind(game.created_at)
        .bind(game.updated_at)
        .fetch_one(exec)
        .await?;

    Ok(game.id)
}

pub async fn select_active_game<'e, E>(exec: E, id: &Uuid) -> Result<ActiveGameRow, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        SELECT id, name, user_id1, user_id2, state, created_at, updated_at
        FROM active_games
        WHERE id = $1;
    "#;

    let game: ActiveGameRow = sqlx::query_as::<_, ActiveGameRow>(query)
        .bind(id)
        .fetch_one(exec)
        .await?;

    Ok(game)
}

pub async fn update_active_game_state<'e, E>(
    exec: E,
    id: &Uuid,
    state: &JsonValue,
) -> Result<ActiveGameRow, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        UPDATE active_games
        SET state = $2
        WHERE id = $1
        RETURNING id, name, user_id1, user_id2, state, created_at, updated_at;
    "#;

    let game: ActiveGameRow = sqlx::query_as::<_, ActiveGameRow>(query)
        .bind(id)
        .bind(state)
        .fetch_one(exec)
        .await?;

    Ok(game)
}

pub async fn listen_active_games_changes(
    pool: &PgPool,
) -> Result<impl Stream<Item = Result<TableChangeEvent<ActiveGameRow>, DatabaseError>>, DatabaseError>
{
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen("active_games_change").await?;

    Ok(stream! {
        while let Some(notification) = listener.try_recv().await? {
            match serde_json::from_str::<TableChangeEvent<ActiveGameRow>>(notification.payload()) {
                Ok(change) => yield Ok(change),
                Err(e) => warn!("Failed to deserialize active_game event: {}", e.to_string()),
            }
        }
    })
}
