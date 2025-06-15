#[cfg(feature = "server")]
use deadpool_redis::{Config as RedisConfig, Pool as RedisPool, Runtime};
#[cfg(feature = "server")]
use sqlx::{migrate, postgres::*};

#[derive(Clone)]
pub struct ApiState {
    pool: PgPool,
    redis: RedisPool,
}

impl ApiState {
    pub async fn setup() -> Result<ApiState, Error> {
        let pool = create_database_pool().await?;
        let redis = create_redis_client().await?;

        Ok(ApiState { pool, redis })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn redis(&self) -> &RedisPool {
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

async fn create_redis_client() -> Result<RedisPool, Error> {
    let redis_url = std::env::var("REDIS_URL")?;

    let config = RedisConfig::from_url(redis_url);
    let pool = config.create_pool(Some(Runtime::Tokio1))?;

    Ok(pool)
}
