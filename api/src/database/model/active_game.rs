use chrono::{DateTime, Utc};
use domain::Game;
use serde::{Serialize, de::DeserializeOwned};
use sqlx::{FromRow, types::Json};
use uuid::Uuid;

#[derive(FromRow)]
pub struct ActiveGameRow<T: Serialize + DeserializeOwned> {
    pub id: Uuid,
    pub name: String,
    pub user_id1: Uuid,
    pub user_id2: Uuid,
    pub state: Json<Game<T>>,
    pub created_at: DateTime<Utc>,
}

impl<T: Serialize + DeserializeOwned> ActiveGameRow<T> {
    pub fn new(
        name: String,
        user_id1: Uuid,
        user_id2: Uuid,
        state: impl Into<Json<Game<T>>>,
    ) -> Self {
        let id = Uuid::new_v4();
        let state = state.into();
        let created_at = Utc::now();

        Self {
            id,
            name,
            user_id1,
            user_id2,
            state,
            created_at,
        }
    }
}
