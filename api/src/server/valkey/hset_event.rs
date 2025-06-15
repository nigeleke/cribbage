use crate::ServiceError;
use deadpool_redis::{Pool, redis::AsyncCommands};
use serde::Serialize;

pub trait HSetEvent {
    async fn hset_event<E: Serialize>(
        &self,
        key: &str,
        field: &str,
        value: &E,
    ) -> Result<String, ServiceError>;
}

impl HSetEvent for Pool {
    async fn hset_event<E: Serialize>(
        &self,
        key: &str,
        field: &str,
        value: &E,
    ) -> Result<String, ServiceError> {
        let mut conn = self.get().await?;

        let value = serde_json::to_string(value)?;

        conn.hset(key, field, value)
            .await
            .map_err(ServiceError::from)
    }
}
