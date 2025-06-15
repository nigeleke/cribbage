use crate::ServiceError;
use deadpool_redis::Pool;

pub trait HDeleteEvents {
    async fn hdelete_events(&self, key: &str) -> Result<usize, ServiceError>;
}

impl HDeleteEvents for Pool {
    async fn hdelete_events(&self, key: &str) -> Result<usize, ServiceError> {
        use redis::AsyncCommands;

        let mut conn = self.get().await?;
        conn.del(key).await.map_err(ServiceError::from)
    }
}
