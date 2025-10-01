use bb8_valkey::ValkeyConnectionManager;
use bb8_valkey::bb8::Pool as ValkeyPool;
use sqlx::postgres::*;
use sqlx::*;

#[derive(Clone)]
pub struct ApiState {
    postgres_pool: PgPool,
    valkey_pool: ValkeyPool<ValkeyConnectionManager>,
}

impl ApiState {
    pub async fn setup() -> Result<ApiState, Error> {
        let postgres_pool = create_postgres_pool().await?;
        let valkey_pool = create_valkey_pool().await?;

        Ok(ApiState {
            postgres_pool,
            valkey_pool,
        })
    }

    pub fn postgres_pool(&self) -> &PgPool {
        &self.postgres_pool
    }

    pub fn valkey_pool(&self) -> &ValkeyPool<ValkeyConnectionManager> {
        &self.valkey_pool
    }
}

type Error = Box<dyn std::error::Error>;

async fn create_postgres_pool() -> Result<PgPool, Error> {
    let database_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await?;

    migrate!().run(&pool).await?;

    Ok(pool)
}

async fn create_valkey_pool() -> Result<ValkeyPool<ValkeyConnectionManager>, Error> {
    let valkey_url = std::env::var("VALKEY_URL")?;

    let manager = ValkeyConnectionManager::new(valkey_url).await?;
    let pool = ValkeyPool::builder().build(manager).await?;

    Ok(pool)
}
