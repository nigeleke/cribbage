use std::ops::DerefMut;

use chrono::{DateTime, Utc};
use sqlx::{Acquire, Postgres, Result};
use uuid::{Timestamp, Uuid};

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

    let rows = sqlx::query_as!(
        AvailableGameRow,
        r#"
        WITH filter AS (
            SELECT
                id::uuid AS id,
                name,
                CASE
                    WHEN host_id = $1 THEN 'Private'
                    WHEN guest_id = $1 THEN 'Private'
                    WHEN guest_id IS NULL THEN 'Public'
                    ELSE 'NotAvailable'
                END AS availability
            FROM games
            WHERE
                ($2::text IS NULL OR name ILIKE $2)
            AND ($3::timestamp with time zone IS NULL
                OR uuid_extract_timestamp(id) < $3)
        )
        SELECT
            id           AS "id!",
            name         AS "name!",
            availability AS "availability!"
        FROM filter
        WHERE availability != 'NotAvailable'
        ORDER BY
            availability = 'Private' DESC,
            id DESC
        "#,
        Some(user_id),
        filter.map(|f| format!("%{}%", f)),
        last_created_at
    )
    .fetch_all(connection.deref_mut())
    .await?;

    let has_more = rows.len() as u32 > chunk_size;
    let games = rows
        .into_iter()
        .take(chunk_size as usize)
        .collect::<Vec<_>>();

    let ts_to_datetime = |ts: Timestamp| {
        let (s, ns) = ts.to_unix();
        DateTime::from_timestamp(s as i64, ns)
    };

    let last_created_at = games
        .last()
        .map(|game| game.id.get_timestamp())
        .and_then(|ts| ts.map(ts_to_datetime))
        .flatten();

    let chunk = AvailableGamesChunk {
        games,
        has_more,
        last_created_at,
    };

    Ok(chunk)
}
