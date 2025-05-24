use redis::{Client, aio::ConnectionManager};
use sqlx::{migrate, postgres::*};

#[derive(Clone)]
pub struct ApiState {
    pool: PgPool,
    redis_client: Client,
    redis: ConnectionManager,
}

impl ApiState {
    pub async fn setup() -> Result<ApiState, Error> {
        let pool = create_database_pool().await?;
        let (redis_client, redis) = create_redis_connection().await?;
        Ok(ApiState {
            pool,
            redis_client,
            redis,
        })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn redis_client(&self) -> &Client {
        &self.redis_client
    }

    pub fn redis(&self) -> &ConnectionManager {
        &self.redis
    }
}

type Error = Box<dyn std::error::Error>;

async fn create_database_pool() -> Result<PgPool, Error> {
    let database_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await?;

    migrate!().run(&pool).await?;

    Ok(pool)
}

async fn create_redis_connection() -> Result<(Client, ConnectionManager), Error> {
    let redis_url = std::env::var("REDIS_URL")?;
    let redis_client = redis::Client::open(redis_url)?;
    let redis = redis_client.get_connection_manager().await?;
    Ok((redis_client, redis))
}
