use crate::domain::prelude::Game;

use leptos::*;
use sqlx::{Any, FromRow, Transaction};
use uuid::Uuid;

use std::ops::DerefMut;

#[derive(Debug, FromRow)]
struct GameRow {
    id: String,
    game: String,
}

pub async fn insert_game(tx: &mut Transaction<'_, Any>, game: &Game) -> Result<Uuid, ServerFnError> {
    logging::log!("db::create_game");

    let id = Uuid::new_v4();

    let game = serde_json::to_string(&game).expect("Failed to serialise game");

    match sqlx::query("INSERT INTO games (id, game) VALUES ($1, $2)")
        .bind(id.to_string())
        .bind(game)
        .execute(tx.deref_mut())
        .await
    {
        Ok(_row) => {
            logging::log!("db::create_game: game_created: {}", id.to_string());
            Ok(id)
        },
        Err(e) => {
            logging::error!("db::create_game: error: {}", e.to_string());
            Err(ServerFnError::ServerError(e.to_string()))
        },
    }
}

pub async fn select_game(tx: &mut Transaction<'_, Any>, id: &Uuid) -> Result<Game, ServerFnError> {
    logging::log!("db::get_game({})", id.to_string());
    match sqlx::query_as::<_, GameRow>("SELECT * FROM games WHERE id == $1")
        .bind(id.to_string())
        .fetch_one(tx.deref_mut())
        .await
    {
        Ok(row) => {
            logging::log!("db::get_game: ok: {:?}", row);
            Ok(serde_json::from_str::<Game>(&row.game).expect("Failed to deserialise game"))
        },
        Err(e) => {
            logging::log!("db::get_game: error: {}", e.to_string());
            Err(ServerFnError::ServerError(e.to_string()))
        },
    }
}
