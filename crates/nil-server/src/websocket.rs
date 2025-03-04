use axum::extract::ws::{Message, WebSocket};
use futures::sink::SinkExt;
use futures::stream::{SplitSink, SplitStream, StreamExt};
use nil_core::Listener;
use nil_util::spawn;
use tokio::select;
use tokio::task::JoinHandle;

type Sender = SplitSink<WebSocket, Message>;
type Receiver = SplitStream<WebSocket>;

pub(crate) async fn handle_socket(socket: WebSocket, listener: Listener) {
  let (socket_tx, socket_rx) = socket.split();
  let mut tx_task = spawn_tx(socket_tx, listener);
  let mut rx_task = spawn_rx(socket_rx);

  select! {
    _ = (&mut tx_task) => rx_task.abort(),
    _ = (&mut rx_task) => tx_task.abort()
  }
}

fn spawn_tx(mut tx: Sender, mut listener: Listener) -> JoinHandle<()> {
  spawn(async move {
    loop {
      // `while let` would break if the listener is lagging behind.
      if let Ok(bytes) = listener.recv().await {
        let _ = tx.send(Message::Binary(bytes)).await;
      }
    }
  })
}

fn spawn_rx(mut rx: Receiver) -> JoinHandle<()> {
  spawn(async move {
    while let Some(Ok(message)) = rx.next().await {
      if let Message::Close(_) = message {
        break;
      }
    }
  })
}
