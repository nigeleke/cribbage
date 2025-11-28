use std::sync::Arc;

use cqrs_es::Query;
use dioxus::fullstack::{FullstackContext, extract::FromRef};
use postgres_es::{PostgresCqrs, PostgresViewRepository, default_postgress_pool, postgres_cqrs};
use sqlx::{PgPool, migrate, postgres::*};
use tokio::sync::broadcast;

use crate::{
    bug,
    database::Notification,
    domain::{Game, GameServices},
    error::ServerError,
    projections::{GameQuery, GameView},
};

#[derive(Clone)]
pub struct ServerState {
    pub pool: Arc<PgPool>,
    pub cqrs: Arc<PostgresCqrs<Game>>,
    pub game_view_repo: Arc<PostgresViewRepository<GameView, Game>>,
    pub database_changes_sender: broadcast::Sender<Notification>,
}

impl ServerState {
    pub fn subscribe_database_changes(&self) -> broadcast::Receiver<Notification> {
        self.database_changes_sender.subscribe()
    }
}

impl FromRef<FullstackContext> for ServerState {
    fn from_ref(state: &FullstackContext) -> Self {
        state
            .extension::<ServerState>()
            .expect("server state must be defined")
    }
}

pub async fn initialize_server_state() -> Result<ServerState, ServerError> {
    let database_url = std::env::var("DATABASE_URL").expect("Database url is not specified");

    let pool = default_postgress_pool(&database_url).await;
    migrate!()
        .run(&pool)
        .await
        .unwrap_or_else(|_| panic!("Failed to migrate data in database: {database_url}"));

    let pool = Arc::new(pool);

    let game_view_repo = Arc::new(PostgresViewRepository::new("game_query", (*pool).clone()));

    let mut game_query = GameQuery::new(game_view_repo.clone());
    game_query.use_error_handler(Box::new(|e| dioxus::prelude::error!("{e}")));

    let queries: Vec<Box<dyn Query<Game>>> = vec![Box::new(game_query)];
    let services = GameServices {};
    let cqrs = postgres_cqrs::<Game>((*pool).clone(), queries, services);
    let cqrs = Arc::new(cqrs);

    let database_changes_sender = create_database_changes_sender((*pool).clone()).await?;

    let state = ServerState {
        pool,
        cqrs,
        game_view_repo,
        database_changes_sender,
    };

    Ok(state)
}

async fn create_database_changes_sender(
    postgres_pool: PgPool,
) -> Result<broadcast::Sender<Notification>, ServerError> {
    let mut listener = PgListener::connect_with(&postgres_pool)
        .await
        .map_err(bug!())?;

    listener
        .listen_all(["game_query_change"])
        .await
        .map_err(bug!())?;

    let (sender, _): (broadcast::Sender<Notification>, _) = broadcast::channel(10);
    let task_sender = sender.clone();

    tokio::spawn(async move {
        loop {
            match listener.recv().await {
                Ok(notification) => {
                    let payload = serde_json::from_str::<Notification>(notification.payload());
                    match payload {
                        Ok(payload) => {
                            let _ = task_sender.send(payload);
                        }
                        Err(error) => {
                            dioxus::prelude::warn!("server_state: {error}");
                        }
                    }
                }
                Err(error) => {
                    dioxus::prelude::error!("server_state:database_listener: failed: {error}");
                    break;
                }
            }
        }
    });

    Ok(sender)
}
