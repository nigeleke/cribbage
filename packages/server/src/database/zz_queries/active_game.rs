use sqlx::types::JsonValue;
use sqlx::{Executor, Postgres, Result};
use uuid::Uuid;

use crate::database::{ActiveGameRow, DatabaseError};

pub async fn insert_active_game<'e, E>(exec: E, game: &ActiveGameRow) -> Result<Uuid, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        INSERT INTO active_games (id, name, host_id, guest_id, state, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, name, host_id, guest_id, state, created_at, updated_at;
    "#;

    let game: ActiveGameRow = sqlx::query_as::<_, ActiveGameRow>(query)
        .bind(game.id)
        .bind(&game.name)
        .bind(game.host_id)
        .bind(game.guest_id)
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
        SELECT id, name, host_id, guest_id, state, created_at, updated_at
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
        RETURNING id, name, host_id, guest_id, state, created_at, updated_at;
    "#;

    let game: ActiveGameRow = sqlx::query_as::<_, ActiveGameRow>(query)
        .bind(id)
        .bind(state)
        .fetch_one(exec)
        .await?;

    Ok(game)
}
