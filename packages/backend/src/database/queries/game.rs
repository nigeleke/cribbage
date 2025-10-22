use sqlx::{Executor, Postgres, Result};
use uuid::Uuid;

use crate::database::{DatabaseError, GameRow, NewGame, UpdateGame};

pub async fn insert_game<'e, E>(exec: E, game: &NewGame) -> Result<Uuid, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        INSERT INTO games (name, host_id, guest_id, state)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, host_id, guest_id, state, created_at, updated_at;
    "#;

    let game: GameRow = sqlx::query_as::<_, GameRow>(query)
        .bind(&game.name)
        .bind(game.host_id)
        .bind(game.guest_id)
        .bind(&game.state)
        .fetch_one(exec)
        .await?;

    Ok(game.id)
}

pub async fn select_game<'e, E>(exec: E, game_id: Uuid) -> Result<Option<GameRow>, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        SELECT id, name, host_id, guest_id, state, created_at, updated_at
        FROM games
        WHERE id = $1
        LIMIT 1;
    "#;

    let game_row = sqlx::query_as::<_, GameRow>(query)
        .bind(game_id)
        .fetch_optional(exec)
        .await?;

    Ok(game_row)
}

pub async fn update_game<'e, E>(exec: E, game: &UpdateGame) -> Result<GameRow, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        UPDATE games
        SET
            name = COALESCE($1, name),
            host_id = COALESCE($2, host_id),
            guest_id = COALESCE($3, guest_id),
            state = COALESCE($4, state)
        WHERE
            id = $5
        RETURNING id, name, host_id, guest_id, state, created_at, updated_at;
    "#;

    let game_row = sqlx::query_as::<_, GameRow>(query)
        .bind(&game.name)
        .bind(game.host_id)
        .bind(game.guest_id)
        .bind(&game.state)
        .bind(game.id)
        .fetch_one(exec)
        .await?;

    Ok(game_row)
}
