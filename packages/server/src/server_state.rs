use crate::{Game, GameServices, projections::GameQuery};
use cqrs_es::QueryWrapper;
use dioxus::fullstack::FullstackContext;
use dioxus::fullstack::extract::FromRef;
use postgres_es::{
    PostgresCqrs, PostgresEventRepository, default_postgress_pool, postgres_aggregate_cqrs,
};
use sqlx::{PgPool, migrate};
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerState {
    pub pool: Arc<PgPool>,
    pub cqrs: Arc<PostgresCqrs<Game>>,
}

impl FromRef<FullstackContext> for ServerState {
    fn from_ref(state: &FullstackContext) -> Self {
        state.extension::<ServerState>().unwrap()
    }
}

pub async fn initialize_server_state() -> ServerState {
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

    ServerState { pool, cqrs }
}
