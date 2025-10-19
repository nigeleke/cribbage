use dioxus::fullstack::Lazy;
use sqlx::migrate;
use sqlx::postgres::*;

use crate::error::BackendError;

#[derive(Clone)]
pub struct ServerState {
    postgres_pool: PgPool,
}

impl ServerState {
    pub async fn setup() -> Result<ServerState, BackendError> {
        let postgres_pool = create_postgres_pool().await?;
        Ok(ServerState { postgres_pool })
    }

    pub fn postgres_pool(&self) -> &PgPool {
        &self.postgres_pool
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

pub static SERVER_STATE: Lazy<ServerState> =
    Lazy::new(|| async move { dioxus::Ok(ServerState::setup().await?) });
