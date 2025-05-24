mod game;

pub use self::game::{insert_game, select_game, update_game};

use sqlx::*;
use sqlx::any::*;

static DB: std::sync::OnceLock<AnyPool> = std::sync::OnceLock::new();

async fn create_pool() -> AnyPool {
    let database_url = std::env::var("DATABASE_URL").expect("Database url is not specified");

    install_default_drivers();

    let pool = AnyPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await
        .expect(&format!("Could not connect to database: {}", database_url));

    migrate!()
        .run(&pool)
        .await
        .expect(&format!("Failed to migrate data in database: {}", database_url));

    pool
}

pub async fn init_database() -> Result<(), Pool<Any>> {
    DB.set(create_pool().await)
}

pub async fn get_database<'a>() -> &'a AnyPool {
    DB.get().expect("Database is not initialized")
}
