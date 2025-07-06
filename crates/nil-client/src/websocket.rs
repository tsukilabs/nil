// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::http::USER_AGENT;
use anyhow::Result as AnyResult;
use bytes::Bytes;
use futures::future::BoxFuture;
use futures::stream::{SplitSink, SplitStream};
use futures::{Sink, SinkExt, StreamExt};
use http::{HeaderValue, header};
use nil_core::event::Event;
use nil_core::player::PlayerId;
use nil_future::task::spawn;
use std::net::SocketAddrV4;
use std::ops::ControlFlow;
use tokio::net::TcpStream;
use tokio::sync::mpsc::channel;
use tokio::task::AbortHandle;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

type ReceiverStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

pub(super) struct WebSocketClient {
  sender: Sender,
  receiver: Receiver,
}

impl WebSocketClient {
  pub async fn connect<F>(addr: &SocketAddrV4, player: &PlayerId, on_event: F) -> Result<Self>
  where
    F: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
  {
    let Ok(authorization) = HeaderValue::from_str(player) else {
      return Err(Error::InvalidPlayerId(player.clone()));
    };

    let ws_stream: AnyResult<WebSocketStream<_>> = try {
      let url = format!("ws://{addr}/ws");
      let mut request = url.into_client_request()?;

      let headers = request.headers_mut();
      headers.insert(header::AUTHORIZATION, authorization);
      headers.insert(header::USER_AGENT, USER_AGENT.parse()?);

      connect_async(request)
        .await
        .map(|(ws_stream, _)| ws_stream)?
    };

    let Ok(ws_stream) = ws_stream else {
      return Err(Error::FailedToConnectWebsocket);
    };

    let (tx, rx) = ws_stream.split();

    Ok(Self {
      sender: Sender::new(tx),
      receiver: Receiver::new(rx, on_event),
    })
  }

  pub(super) fn stop(self) {
    self.sender.stop();
    self.receiver.stop();
  }
}

struct Sender {
  ws_sender_handle: AbortHandle,
  keep_alive_handle: AbortHandle,
}

impl Sender {
  fn new<T>(mut ws_sender: SplitSink<T, Message>) -> Self
  where
    T: Sink<Message> + Send + 'static,
  {
    let (tx, mut rx) = channel::<SenderMessage>(10);
    let ws_sender_task = spawn(async move {
      while let Some(message) = rx.recv().await {
        if message.send(&mut ws_sender).await.is_break() {
          break;
        }
      }
    });

    let keep_alive_task = spawn(async move {
      while let Ok(()) = tx.send(SenderMessage::KeepAlive).await {
        sleep(Duration::from_secs(30)).await;
      }
    });

    Self {
      ws_sender_handle: ws_sender_task.abort_handle(),
      keep_alive_handle: keep_alive_task.abort_handle(),
    }
  }

  fn stop(self) {
    self.ws_sender_handle.abort();
    self.keep_alive_handle.abort();
  }
}

enum SenderMessage {
  KeepAlive,
}

impl SenderMessage {
  async fn send<T>(self, ws_sender: &mut SplitSink<T, Message>) -> ControlFlow<()>
  where
    T: Sink<Message>,
  {
    match self {
      SenderMessage::KeepAlive => {
        let _ = ws_sender
          .send(Message::Ping(Bytes::default()))
          .await;
      }
    }

    ControlFlow::Continue(())
  }
}

struct Receiver {
  ws_receiver_handle: AbortHandle,
}

impl Receiver {
  fn new<F>(mut ws_receiver: ReceiverStream, on_event: F) -> Self
  where
    F: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
  {
    let ws_receiver_task = spawn(async move {
      while let Some(Ok(message)) = ws_receiver.next().await {
        if let Message::Binary(bytes) = message {
          on_event(Event::from(bytes)).await;
        } else if let Message::Close(_) = message {
          break;
        }
      }
    });

    Self {
      ws_receiver_handle: ws_receiver_task.abort_handle(),
    }
  }

  fn stop(self) {
    self.ws_receiver_handle.abort();
  }
}
