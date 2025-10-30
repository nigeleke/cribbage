use dioxus::fullstack::Lazy;
use dioxus::prelude::*;
use sqlx::migrate;
use sqlx::postgres::*;
use tokio::sync::broadcast;

use crate::database::Notification;
use crate::error::BackendError;

pub struct ServerState {
    postgres_pool: PgPool,
    database_changes_sender: broadcast::Sender<Notification>,
}

impl ServerState {
    pub async fn setup() -> Result<ServerState, BackendError> {
        let postgres_pool = create_postgres_pool().await?;
        let database_changes_sender = create_database_changes_sender(&postgres_pool).await?;
        Ok(ServerState {
            postgres_pool,
            database_changes_sender,
        })
    }

    pub(crate) fn postgres_pool(&self) -> &PgPool {
        &self.postgres_pool
    }

    pub(crate) fn subscribe_database_changes(&self) -> broadcast::Receiver<Notification> {
        self.database_changes_sender.subscribe()
    }
}

async fn create_postgres_pool() -> Result<PgPool, BackendError> {
    let database_url = std::env::var("DATABASE_URL")?;

    let postgres_pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(10)
        .connect(&database_url)
        .await?;

    migrate!("./migrations").run(&postgres_pool).await?;

    Ok(postgres_pool)
}

async fn create_database_changes_sender(
    postgres_pool: &PgPool,
) -> Result<broadcast::Sender<Notification>, BackendError> {
    let mut listener = PgListener::connect_with(postgres_pool).await?;
    listener.listen_all(["games_change"]).await?;

    let (tx, _rx) = broadcast::channel::<Notification>(10);

    let tx_emitter = tx.clone();
    tokio::spawn(async move {
        loop {
            match listener.recv().await {
                Ok(notification) => {
                    let result = serde_json::from_str::<Notification>(notification.payload());
                    match result {
                        Ok(payload) => {
                            let _ = tx_emitter.send(payload);
                        }
                        Err(e) => {
                            error!("Failed parsing notification payload: {e}");
                        }
                    }
                }
                Err(e) => {
                    error!("database listener failed: {e}");
                    break;
                }
            }
        }
    });

    Ok(tx)
}

pub static SERVER_STATE: Lazy<ServerState> =
    Lazy::new(|| async move { ServerState::setup().await });
