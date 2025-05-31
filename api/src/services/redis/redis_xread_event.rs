use redis::{
    AsyncCommands, ErrorKind, FromRedisValue, RedisError, RedisResult, Value,
    streams::StreamReadOptions,
};
use serde::de::DeserializeOwned;

#[derive(Debug)]
struct StreamMessage<E: DeserializeOwned + std::fmt::Debug> {
    _message_id: String,
    messages: Vec<(String, E)>,
}

#[derive(Debug)]
struct StreamEntry<E: DeserializeOwned + std::fmt::Debug> {
    stream_name: String,
    entries: Vec<StreamMessage<E>>,
}

impl<E: DeserializeOwned + std::fmt::Debug> FromRedisValue for StreamEntry<E> {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
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
            stream_name: stream_name.to_string(),
            entries: Vec::from([StreamMessage {
                _message_id: message_id.to_string(),
                messages: messages
                    .chunks_exact(2)
                    .filter_map(|c| {
                        let entry = serde_json::from_str(&c[1]).ok();
                        entry.map(|e| (c[0].clone(), e))
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
    ) -> RedisResult<E>;
}

impl<T: AsyncCommands> XReadEvent for T {
    async fn xread_message<E: DeserializeOwned + Clone + std::fmt::Debug>(
        &mut self,
        stream: &str,
    ) -> RedisResult<E> {
        let value: RedisResult<Value> = self
            .xread_options(
                &[stream],
                &["$"],
                &StreamReadOptions::default().count(1).block(0),
            )
            .await;

        match value {
            Ok(value) => {
                let entry = StreamEntry::<E>::from_redis_value(&value)?;
                if entry.stream_name != stream {
                    let error = format!("unexpected stream: {}", entry.stream_name);
                    Err(redis_error(&error))
                } else {
                    let event = entry.entries[0].messages[0].1.clone();
                    Ok(event)
                }
            }
            Err(e) => Err(e),
        }
    }
}
