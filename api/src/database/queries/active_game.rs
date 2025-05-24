use serde::{Serialize, de::DeserializeOwned};
use sqlx::{Executor, Postgres, Result};

use crate::{ActiveGameId, DatabaseError, database::model::ActiveGameRow};

pub async fn insert_active_game<'e, T, E>(
    exec: E,
    game: &ActiveGameRow<T>,
) -> Result<ActiveGameId, DatabaseError>
where
    T: Serialize + DeserializeOwned + Send + Unpin + 'static,
    E: Executor<'e, Database = Postgres>,
{
    let query = r#"
        INSERT INTO active_games (id, name, user_id1, user_id2, state, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, user_id1, user_id2, state, created_at;
    "#;

    let game: ActiveGameRow<T> = sqlx::query_as::<_, ActiveGameRow<T>>(query)
        .bind(game.id)
        .bind(&game.name)
        .bind(game.user_id1)
        .bind(game.user_id2)
        .bind(&game.state)
        .bind(game.created_at)
        .fetch_one(exec)
        .await?;

    Ok(ActiveGameId::from(game.id))
}
