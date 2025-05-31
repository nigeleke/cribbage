use redis::{AsyncCommands, RedisResult};
use serde::Serialize;

pub trait XAddEvent {
    async fn xadd_message<E: Serialize>(&mut self, stream: &str, event: &E) -> RedisResult<String>;
}

impl<T: AsyncCommands> XAddEvent for T {
    async fn xadd_message<E: Serialize>(&mut self, stream: &str, event: &E) -> RedisResult<String> {
        let payload = serde_json::to_string(&event)?;
        self.xadd(stream, "*", &[("message", payload)]).await
    }
}
