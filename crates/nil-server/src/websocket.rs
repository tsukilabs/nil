#![allow(unused_imports, unused_mut, unused_variables)]

use axum::body::Bytes;
use axum::extract::ws::{Message, WebSocket};
use futures::sink::SinkExt;
use futures::stream::{SplitSink, SplitStream, StreamExt};
use std::net::SocketAddr;
use std::ops::ControlFlow;
use tauri::async_runtime::{JoinHandle, spawn};
use tokio::select;
use tokio::sync::mpsc::channel;
use tokio::time::{Duration, sleep};

#[cfg(feature = "tracing")]
use tracing::info;

pub(crate) async fn handle_socket(mut socket: WebSocket, addr: SocketAddr) {
  let (socket_tx, socket_rx) = socket.split();
  let mut tx_task = spawn_sender_task(socket_tx, addr);
  let mut rx_task = spawn_receiver_task(socket_rx, addr);

  select! {
    _ = (&mut tx_task) => {
      #[cfg(feature = "tracing")]
      info!("sender task for {addr} finished");

      rx_task.abort();
    }
    _ = (&mut rx_task) => {
      #[cfg(feature = "tracing")]
      info!("receiver task for {addr} finished");

      tx_task.abort();
    }
  }
}

fn spawn_sender_task(mut tx: SplitSink<WebSocket, Message>, addr: SocketAddr) -> JoinHandle<()> {
  spawn(async move {
    let mut n = 0;
    loop {
      n += 1;
      let message = format!("{n}: hello, {addr}!");
      if tx
        .send(Message::Text(message.into()))
        .await
        .is_err()
      {
        #[cfg(feature = "tracing")]
        info!("client {addr} disconnected");

        break;
      }

      sleep(Duration::from_secs(2)).await;
    }
  })
}

fn spawn_receiver_task(mut rx: SplitStream<WebSocket>, addr: SocketAddr) -> JoinHandle<()> {
  spawn(async move {
    while let Some(Ok(message)) = rx.next().await {
      if let Message::Close(_) = message {
        #[cfg(feature = "tracing")]
        info!("client {addr} disconnected");

        break;
      }
    }
  })
}
