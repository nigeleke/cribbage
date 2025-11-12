use crate::database::{DatabaseError, Notification};
use crate::error::ServerError;
use crate::{domain::Game, domain::GameServices, projections::GameQuery};

use cqrs_es::QueryWrapper;
use dioxus::fullstack::FullstackContext;
use dioxus::fullstack::extract::FromRef;
use dioxus::prelude::*;
use postgres_es::{PostgresCqrs, default_postgress_pool, postgres_aggregate_cqrs};
use sqlx::postgres::*;
use sqlx::{PgPool, migrate};
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct ServerState {
    pub pool: Arc<PgPool>,
    pub cqrs: Arc<PostgresCqrs<Game>>,
    pub database_changes_sender: broadcast::Sender<Notification>,
}

impl ServerState {
    pub fn subscribe_database_changes(&self) -> broadcast::Receiver<Notification> {
        self.database_changes_sender.subscribe()
    }
}

impl FromRef<FullstackContext> for ServerState {
    fn from_ref(state: &FullstackContext) -> Self {
        state.extension::<ServerState>().unwrap()
    }
}

pub async fn initialize_server_state() -> Result<ServerState, ServerError> {
    let database_url = std::env::var("DATABASE_URL").expect("Database url is not specified");

    let pool = default_postgress_pool(&database_url).await;
    migrate!().run(&pool).await.expect(&format!(
        "Failed to migrate data in database: {}",
        database_url
    ));

    let pool = Arc::new(pool);

    let queries: Vec<QueryWrapper<Game>> = vec![QueryWrapper::new(GameQuery::new(pool.clone()))];
    let services = GameServices {};
    let cqrs = postgres_aggregate_cqrs((*pool).clone(), queries, services);
    let cqrs = Arc::new(cqrs);

    let database_changes_sender = create_database_changes_sender((*pool).clone()).await?;

    let state = ServerState {
        pool,
        cqrs,
        database_changes_sender,
    };

    Ok(state)
}

async fn create_database_changes_sender(
    postgres_pool: PgPool,
) -> Result<broadcast::Sender<Notification>, ServerError> {
    let mut listener = PgListener::connect_with(&postgres_pool)
        .await
        .map_err(DatabaseError::from)?;
    listener
        .listen_all(["events_change", "games_change"])
        .await
        .map_err(DatabaseError::from)?;

    let (tx, _rx): (broadcast::Sender<Notification>, _) = broadcast::channel(10);
    let tx2 = tx.clone();
    tokio::spawn(async move {
        loop {
            match listener.recv().await {
                Ok(notification) => {
                    let payload = serde_json::from_str::<Notification>(notification.payload());
                    if let Ok(parsed) = payload {
                        debug!("server_state:database_listener: received: {parsed:?}");
                        let _ = tx2.send(parsed);
                    }
                }
                Err(e) => {
                    error!("server_state:database_listener: failed: {e}");
                    break;
                }
            }
        }
    });

    Ok(tx)
}
