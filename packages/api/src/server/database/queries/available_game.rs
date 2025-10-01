use chrono::{DateTime, Utc};
use sqlx::{Executor, Postgres, Result};
use uuid::Uuid;

use crate::server::database::{AvailableGameRow, DatabaseError};

pub struct AvailableGamesChunk {
    pub games: Vec<AvailableGameRow>,
    pub has_more: bool,
    pub last_created_at: Option<DateTime<Utc>>,
}

pub async fn select_available_games<'e, E>(
    exec: E,
    chunk_size: u32,
    last_created_at: Option<DateTime<Utc>>,
    filter: Option<String>,
    user_id: Uuid,
) -> Result<AvailableGamesChunk, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let filter_pattern = filter.map(|f| format!("%{}%", f));

    let query = r#"
        SELECT id, user_id, source, name, created_at
        FROM available_games
        WHERE (
            $1::timestamp with time zone IS NULL
            OR created_at > $1
        )
        AND ($2::text IS NULL OR name ILIKE $2)
        AND ((user_id != $3 AND source = 'Lobby')
            OR
            (user_id = $3 AND source = 'Active'))
        ORDER BY
            CASE WHEN source = 'Active' THEN 0 ELSE 1 END,
            CASE WHEN source = 'Active' THEN created_at END DESC,
            CASE WHEN source = 'Lobby' THEN created_at END ASC
        LIMIT $4
    "#;

    let rows = sqlx::query_as::<_, AvailableGameRow>(query)
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

    let chunk = AvailableGamesChunk {
        games,
        has_more,
        last_created_at,
    };

    Ok(chunk)
}
