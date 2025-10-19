use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::{Stream, StreamExt};
use sqlx::postgres::PgListener;
use sqlx::{Acquire, Executor, Postgres, Result};
use tokio::sync::mpsc;

use crate::database::{DatabaseError, Notification};

pub struct ChangeStream {
    receiver: mpsc::UnboundedReceiver<Notification>,
}

impl Stream for ChangeStream {
    type Item = Result<Notification, DatabaseError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.receiver.poll_recv(cx) {
            Poll::Ready(Some(notification)) => Poll::Ready(Some(Ok(notification))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub async fn change_stream<'e, E>(exec: E, channel: &str) -> Result<ChangeStream, DatabaseError>
where
    E: Acquire<'e, Database = Postgres>,
{
    let listener = PgListener::connect_with(&exec).await?;

    let query = format!("LISTEN {};", channel);
    let listener = sqlx::query(&query).execute(exec).await?;

    dioxus::logger::tracing::info!("generic_database::change_stream {channel}: {listener:?}");

    let (_tx, rx) = mpsc::unbounded_channel();

    // tokio::spawn(async move {
    //     let mut notifications = exec.;
    //     while let Some(notification) = notifications.next().await {
    //         match notification {
    //             Ok(Notification { payload, .. }) => {
    //                 // Deserialize the JSON payload into a Game struct
    //                 match serde_json::from_str::<GameRow>(&payload) {
    //                     Ok(game) => {
    //                         if tx.send(game).is_err() {
    //                             eprintln!("Receiver dropped, stopping notification processing");
    //                             break;
    //                         }
    //                     }
    //                     Err(e) => eprintln!("Failed to deserialize payload: {}", e),
    //                 }
    //             }
    //             Err(e) => {
    //                 eprintln!("Notification error: {}", e);
    //                 break;
    //             }
    //         }
    //     }
    // });

    Ok(ChangeStream { receiver: rx })
}
