use std::ops::DerefMut;

use chrono::{DateTime, Utc};
use sqlx::{Acquire, Postgres, Result};
use uuid::Uuid;

use crate::database::AvailableGameRow;

pub struct AvailableGamesChunk {
    pub games: Vec<AvailableGameRow>,
    pub has_more: bool,
    pub last_created_at: Option<DateTime<Utc>>,
}

pub async fn select_available_games<'a, A>(
    acquirer: A,
    chunk_size: u32,
    last_created_at: Option<DateTime<Utc>>,
    filter: Option<String>,
    user_id: Uuid,
) -> Result<AvailableGamesChunk>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut connection = acquirer.acquire().await?;

    let filter_pattern = filter.map(|f| format!("%{}%", f));

    let rows = sqlx::query_as!(
        AvailableGameRow,
        r#"WITH filter AS (
            SELECT
                id,
                name,
                CASE
                    WHEN host_id = $1 THEN 'Private'
                    WHEN guest_id = $1 THEN 'Private'
                    WHEN (host_id != $1 AND guest_id IS NULL) THEN 'Public'
                    ELSE 'NotAvailable'
                END AS "availability!",
                created_at
            FROM games
            WHERE
                ($2::text IS NULL OR name ILIKE $2)
            AND ($3::timestamp with time zone IS NULL
                OR created_at < $3)
        )
        SELECT *
        FROM filter
        WHERE "availability!" != 'NotAvailable'
        ORDER BY
            CASE WHEN "availability!" = 'Private' THEN 0 ELSE 1 END,
            created_at DESC"#,
        user_id,
        filter_pattern,
        last_created_at
    )
    .fetch_all(connection.deref_mut())
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
