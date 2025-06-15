use crate::ServiceError;
use serde::Serialize;

pub trait XAddEvent {
    async fn xadd_message<E: Serialize>(
        &self,
        stream: &str,
        event: &E,
    ) -> Result<String, ServiceError>;
}

impl XAddEvent for Pool {
    async fn xadd_message<E: Serialize>(
        &self,
        stream: &str,
        event: &E,
    ) -> Result<String, ServiceError> {
        use valkey::AsyncCommands;

        let payload = serde_json::to_string(&event)?;

        let mut conn = self.get().await?;

        let result = conn.xadd(stream, "*", &[("message", &payload)]).await?;
        println!("XAdd result: {result}");
        Ok(result)
    }
}
