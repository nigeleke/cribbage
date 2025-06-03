use crate::database::{DatabaseError, TableChangeEvent, UnstartedGameRow};
use async_stream::stream;
use dioxus::logger::tracing::warn;
use futures_util::Stream;
use sqlx::{Executor, PgPool, Postgres, Result, postgres::PgListener};
use uuid::Uuid;

pub async fn insert_unstarted_game<'e, E>(
    exec: E,
    game: &UnstartedGameRow,
) -> Result<UnstartedGameRow, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        INSERT INTO unstarted_games (id, owner_id, name, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id, owner_id, name, created_at;
    "#;

    let game = sqlx::query_as::<_, UnstartedGameRow>(query)
        .bind(game.id)
        .bind(game.owner_id)
        .bind(&game.name)
        .bind(game.created_at)
        .fetch_one(exec)
        .await?;

    Ok(game)
}

pub async fn select_unstarted_game<'e, E>(
    exec: E,
    id: Uuid,
) -> Result<UnstartedGameRow, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        SELECT id, owner_id, name, created_at
        FROM unstarted_games
        WHERE (id = $1);
    "#;

    let game = sqlx::query_as::<_, UnstartedGameRow>(query)
        .bind(id)
        .fetch_one(exec)
        .await?;

    Ok(game)
}

pub async fn delete_unstarted_game<'e, E>(exec: E, game_id: Uuid) -> Result<(), DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        DELETE FROM unstarted_games
        WHERE id = $1;
    "#;

    sqlx::query(query).bind(game_id).execute(exec).await?;

    Ok(())
}

pub async fn listen_unstarted_games_changes(
    pool: &PgPool,
) -> Result<
    impl Stream<Item = Result<TableChangeEvent<UnstartedGameRow>, DatabaseError>>,
    DatabaseError,
> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen("unstarted_games_change").await?;

    Ok(stream! {
        while let Some(notification) = listener.try_recv().await? {
            match serde_json::from_str::<TableChangeEvent<UnstartedGameRow>>(notification.payload()) {
                Ok(change) => yield Ok(change),
                Err(e) => warn!("Failed to deserialize unstarted_game event: {}", e.to_string()),
            }
        }
    })
}
