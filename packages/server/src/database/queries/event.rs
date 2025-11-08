use sqlx::types::JsonValue;
use sqlx::{PgConnection, QueryBuilder};
use uuid::Uuid;

pub async fn append_events(
    connection: &mut PgConnection,
    aggregate_id: Uuid,
    mut events: Vec<JsonValue>,
) -> Result<(), sqlx::Error> {
    if events.is_empty() {
        return Ok(());
    }

    let query = r#"
        SELECT sequence
        FROM events
        WHERE aggregate_id = $1
        ORDER BY sequence DESC
        LIMIT 1
        FOR UPDATE
    "#;

    let max_seq: Option<i64> = sqlx::query_scalar(query)
        .bind(aggregate_id)
        .fetch_optional(&mut *connection)
        .await?;

    let query = r#"INSERT INTO events (aggregate_id, sequence, payload)"#;
    let mut builder = QueryBuilder::new(query);

    let next_seq = max_seq.map(|s| s + 1).unwrap_or(1);
    let query = builder
        .push_values(events.drain(..).enumerate(), |mut builder, (i, event)| {
            let seq = next_seq + i as i64;

            builder
                .push_bind(aggregate_id)
                .push_bind(seq)
                .push_bind(event);
        })
        .build();

    query.execute(&mut *connection).await?;

    Ok(())
}
