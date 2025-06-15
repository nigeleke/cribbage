use crate::ServiceError;
use deadpool_redis::{
    Pool,
    redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, Value},
};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;

#[derive(Debug)]
struct StreamMessage {
    _message_id: String,
    messages: Vec<(String, JsonValue)>,
}

#[derive(Debug)]
struct StreamEntry {
    _stream_name: String,
    entries: Vec<StreamMessage>,
}

impl FromRedisValue for StreamEntry {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        use std::str::FromStr;

        println!("FromRedisValue: {:?}", value);

        let Value::Array(items) = value else {
            return Err(redis_error("missing items"));
        };

        let Value::Array(streams) = &items[0] else {
            return Err(redis_error("no streams"));
        };

        let Value::BulkString(stream_name) = &streams[0] else {
            return Err(redis_error("no stream"));
        };

        let stream_name = String::from_utf8_lossy(stream_name);

        let Value::Array(messages) = &streams[1] else {
            return Err(redis_error("no message groups"));
        };

        let Value::Array(messages) = &messages[0] else {
            return Err(redis_error("no messages"));
        };

        let Value::BulkString(message_id) = &messages[0] else {
            return Err(redis_error("no message id"));
        };

        let message_id = String::from_utf8_lossy(message_id);

        let Value::Array(messages) = &messages[1] else {
            return Err(redis_error("no message"));
        };

        let messages = messages
            .iter()
            .filter_map(|m| {
                if let Value::BulkString(v) = m {
                    Some(String::from_utf8_lossy(&v).to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let entry = StreamEntry {
            _stream_name: stream_name.to_string(),
            entries: Vec::from([StreamMessage {
                _message_id: message_id.to_string(),
                messages: messages
                    .chunks_exact(2)
                    .filter_map(|c| {
                        let key = &c[0];
                        let json = &c[1];

                        println!("XRead json: {json}");

                        let json = json.trim_matches(['"']);
                        let json = json.replace("\\\"", "\"");

                        let result = JsonValue::from_str(&json);
                        result.ok().map(|e| (key.clone(), e))
                    })
                    .collect::<Vec<_>>(),
            }]),
        };

        Ok(entry)
    }
}

pub(crate) fn redis_error(error: &str) -> RedisError {
    let error = String::from(error);
    RedisError::from((
        ErrorKind::ExtensionError,
        "redis deserialisation error",
        error,
    ))
}

pub trait XReadEvent {
    async fn xread_message<E: DeserializeOwned + Clone + std::fmt::Debug>(
        &mut self,
        stream: &str,
    ) -> Result<E, ServiceError>;
}

impl XReadEvent for Pool {
    async fn xread_message<E: DeserializeOwned + Clone + std::fmt::Debug>(
        &mut self,
        stream: &str,
    ) -> Result<E, ServiceError> {
        use redis::{AsyncCommands, streams::StreamReadOptions};

        let mut conn = self.get().await?;

        let options = StreamReadOptions::default().count(1).block(0);

        let value = conn.xread_options(&[stream], &["$"], &options).await;

        match value {
            Ok(value) => {
                let entry = StreamEntry::from_redis_value(&value)?;
                let json = entry.entries[0].messages[0].1.clone();
                let event = serde_json::from_value::<E>(json)?;
                Ok(event)
            }
            Err(e) => Err(ServiceError::Redis(e)),
        }
    }
}
