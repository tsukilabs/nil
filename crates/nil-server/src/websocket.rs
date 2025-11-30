// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use axum::extract::ws::{Message, WebSocket};
use futures::sink::SinkExt;
use futures::stream::{SplitSink, SplitStream, StreamExt};
use nil_core::event::{EventTarget, Listener};
use nil_core::player::PlayerId;
use tokio::select;
use tokio::task::JoinHandle;

type Sender = SplitSink<WebSocket, Message>;
type Receiver = SplitStream<WebSocket>;

pub(crate) async fn handle_socket(socket: WebSocket, listener: Listener, player: PlayerId) {
  let (socket_tx, socket_rx) = socket.split();
  let mut tx_task = spawn_tx(socket_tx, listener, player);
  let mut rx_task = spawn_rx(socket_rx);

  select! {
    _ = (&mut tx_task) => rx_task.abort(),
    _ = (&mut rx_task) => tx_task.abort()
  }
}

fn spawn_tx(mut tx: Sender, mut listener: Listener, player: PlayerId) -> JoinHandle<()> {
  tokio::spawn(async move {
    loop {
      if let Ok((bytes, target)) = listener.recv().await {
        match target {
          EventTarget::Broadcast => {
            let _ = tx.send(Message::Binary(bytes)).await;
          }
          EventTarget::Player(id) if id == player => {
            let _ = tx.send(Message::Binary(bytes)).await;
          }
          _ => {}
        }
      }
    }
  })
}

fn spawn_rx(mut rx: Receiver) -> JoinHandle<()> {
  tokio::spawn(async move {
    while let Some(Ok(message)) = rx.next().await {
      if let Message::Close(_) = message {
        break;
      }
    }
  })
}
