use crate::ServiceError;
use deadpool_redis::Pool;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub trait HGetEvents {
    async fn hget_events<E: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<HashMap<String, E>, ServiceError>;
}

impl HGetEvents for Pool {
    async fn hget_events<E: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<HashMap<String, E>, ServiceError> {
        use redis::AsyncCommands;

        let mut conn = self.get().await?;

        let values: Vec<(String, String)> = conn.hgetall(key).await?;

        println!("HGet: {:#?}", values);

        values
            .into_iter()
            .map(|(field, value)| {
                serde_json::from_str::<E>(&value)
                    .map(|deserialized| (field, deserialized))
                    .map_err(ServiceError::from)
            })
            .collect::<Result<HashMap<String, E>, ServiceError>>()
    }
}
