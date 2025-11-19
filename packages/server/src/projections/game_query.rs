use crate::database::{self, NewGame};
use crate::domain::{Game, GameEvent};
use crate::{
    convertors::{game_row_to_game, state_to_json},
    error::ServerError,
};
use cqrs_es::{EventEnvelope, Query};
use dioxus::prelude::*;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct GameQuery {
    pool: Arc<PgPool>,
}

impl GameQuery {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    async fn try_dispatch(
        pool: Arc<PgPool>,
        aggregate_id: &str,
        events: &[GameEvent],
    ) -> Result<(), ServerError> {
        let mut tx = pool.begin().await.map_err(ServerError::bug)?;

        let game_id = Uuid::from_str(aggregate_id).map_err(ServerError::bug)?;

        let game = database::select_game(&mut *tx, game_id)
            .await
            .map_err(ServerError::bug)?;

        let mut game = match game {
            None => Game::default(),
            Some(game) => game_row_to_game(game).map_err(ServerError::bug)?,
        };

        game.apply_events(events);

        let new_game = NewGame {
            id: game_id,
            name: game.name().clone(),
            host_id: game.host().value(),
            guest_id: game.guest().map(|g| g.value()),
            state: state_to_json(game.state()),
        };

        database::upsert_game(&mut *tx, &new_game)
            .await
            .map_err(ServerError::bug)?;

        tx.commit().await.map_err(ServerError::bug)?;
        Ok(())
    }
}

impl Query<Game> for GameQuery {
    async fn dispatch<'a, 'b, 'c>(
        &'a self,
        aggregate_id: &'b str,
        events: &'c [EventEnvelope<Game>],
    ) {
        let events = events
            .iter()
            .map(|e| &e.payload)
            .cloned()
            .collect::<Vec<_>>();

        let pool = self.pool.clone();
        if let Err(error) = GameQuery::try_dispatch(pool, aggregate_id, &events).await {
            error!("Failed to apply events: {error}");
        };
    }
}
