use sqlx::{Executor, Postgres, Result};
use uuid::Uuid;

use crate::server::database::{DatabaseError, LobbyGameRow};

pub async fn insert_lobby_game<'e, E>(
    exec: E,
    game: &LobbyGameRow,
) -> Result<LobbyGameRow, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        INSERT INTO lobby_games (id, host_id, name, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id, host_id, name, created_at;
    "#;

    let game = sqlx::query_as::<_, LobbyGameRow>(query)
        .bind(game.id)
        .bind(game.host_id)
        .bind(&game.name)
        .bind(game.created_at)
        .fetch_one(exec)
        .await?;

    Ok(game)
}

pub async fn select_lobby_game<'e, E>(exec: E, id: Uuid) -> Result<LobbyGameRow, DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        SELECT id, host_id, name, created_at
        FROM lobby_games
        WHERE (id = $1);
    "#;

    let game = sqlx::query_as::<_, LobbyGameRow>(query)
        .bind(id)
        .fetch_one(exec)
        .await?;

    Ok(game)
}

pub async fn delete_lobby_game<'e, E>(exec: E, game_id: Uuid) -> Result<(), DatabaseError>
where
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        DELETE FROM lobby_games
        WHERE id = $1;
    "#;

    sqlx::query(query).bind(game_id).execute(exec).await?;

    Ok(())
}
