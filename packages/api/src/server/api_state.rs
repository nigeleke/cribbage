use sqlx::postgres::*;
use sqlx::*;

#[derive(Clone)]
pub struct ApiState {
    postgres_pool: PgPool,
}

impl ApiState {
    pub async fn setup() -> Result<ApiState, Error> {
        let postgres_pool = create_postgres_pool().await?;

        Ok(ApiState { postgres_pool })
    }

    pub fn postgres_pool(&self) -> &PgPool {
        &self.postgres_pool
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
