use volga::{di::Dc, ws::WebSocket};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tungstenite::Message;
use super::users::Users;

pub(super) async fn user_connected(ws: WebSocket, users: Dc<Users>) {
    let (mut user_tx, mut user_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_tx
                .send(message)
                .unwrap_or_else(|err| tracing::error!("websocket send error: {}", err))
                .await;
        }
    });

    let id = users.add(tx).await;

    while let Some(result) = user_rx.next().await {
        match result {
            Ok(msg) => handle_message(id, msg, &users).await,
            Err(e) => {
                tracing::error!("websocket error(uid={}): {}", id, e);
                break;
            }
        };
    }

    user_disconnected(id, &users).await;
}

async fn user_disconnected(id: usize, users: &Users) {
    tracing::trace!("user disconnected: {}", id);
    users.remove(id).await;
}

async fn handle_message(id: usize, msg: Message, users: &Users) {
    let new_msg = format!("[User#{}]: {}", id, msg);
    for (&uid, tx) in users.connections.read().await.iter() {
        if id != uid {
            if let Err(_err) = tx.send(new_msg.clone().into()) {}
        }
    }
}