use sqlx::{Executor, Postgres, Result};
use uuid::Uuid;

use crate::database::{DatabaseError, GameRow, NewGame};

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

// pub async fn game_change_stream<'e, E>(exec: E) -> Result<ChangeStream, DatabaseError>
// where
//     E: Executor<'e, Database = Postgres>,
// {
//     change_stream(exec, "notify_games_change").await?

//     // let (tx, rx) = mpsc::unbounded_channel();

//     // tokio::spawn(async move {
//     //     let mut notifications = exec.;
//     //     while let Some(notification) = notifications.next().await {
//     //         match notification {
//     //             Ok(Notification { payload, .. }) => {
//     //                 // Deserialize the JSON payload into a Game struct
//     //                 match serde_json::from_str::<GameRow>(&payload) {
//     //                     Ok(game) => {
//     //                         if tx.send(game).is_err() {
//     //                             eprintln!("Receiver dropped, stopping notification processing");
//     //                             break;
//     //                         }
//     //                     }
//     //                     Err(e) => eprintln!("Failed to deserialize payload: {}", e),
//     //                 }
//     //             }
//     //             Err(e) => {
//     //                 eprintln!("Notification error: {}", e);
//     //                 break;
//     //             }
//     //         }
//     //     }
//     // });

//     // Ok(GameChangeStream { receiver: rx })
// }
