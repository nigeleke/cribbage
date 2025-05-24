use crate::database::{DatabaseError, TableChangeEvent, UnstartedGameRow};
use async_stream::stream;
use chrono::{DateTime, Utc};
use dioxus::logger::tracing::warn;
use futures::Stream;
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

pub struct UnstartedGamesChunk {
    pub games: Vec<UnstartedGameRow>,
    pub has_more: bool,
    pub last_created_at: Option<DateTime<Utc>>,
}

pub async fn select_unstarted_games<'e, E>(
    exec: E,
    chunk_size: u32,
    last_created_at: Option<DateTime<Utc>>,
    filter: Option<String>,
    user_id: Uuid,
) -> Result<UnstartedGamesChunk, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let filter_pattern = filter.map(|f| format!("%{}%", f));

    let query = r#"
        SELECT id, owner_id, name, created_at
        FROM unstarted_games
        WHERE (
            $1::timestamp with time zone IS NULL
            OR created_at > $1
        )
        AND ($2::text IS NULL OR name ILIKE $2)
        AND owner_id != $3
        ORDER BY created_at
        LIMIT $4
    "#;

    let rows = sqlx::query_as::<_, UnstartedGameRow>(query)
        .bind(last_created_at)
        .bind(filter_pattern)
        .bind(user_id)
        .bind(chunk_size as i64 + 1)
        .fetch_all(exec)
        .await?;

    let has_more = rows.len() as u32 > chunk_size;
    let games = rows
        .into_iter()
        .take(chunk_size as usize)
        .collect::<Vec<_>>();

    let last_created_at = games.last().map(|game| game.created_at);

    let chunk = UnstartedGamesChunk {
        games,
        has_more,
        last_created_at,
    };

    Ok(chunk)
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
