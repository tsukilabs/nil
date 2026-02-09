// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::http::USER_AGENT;
use crate::http::authorization::Authorization;
use crate::server::ServerAddr;
use anyhow::Result as AnyResult;
use bytes::Bytes;
use either::Either;
use futures::future::BoxFuture;
use futures::stream::{SplitSink, SplitStream};
use futures::{Sink, SinkExt, StreamExt};
use http::{Method, header};
use nil_core::event::Event;
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use std::ops::ControlFlow;
use tokio::net::TcpStream;
use tokio::spawn;
use tokio::sync::mpsc::channel;
use tokio::task::AbortHandle;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::tungstenite::Error as TungsteniteError;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub(super) struct WebSocketClient {
  _sender: Sender,
  _receiver: Receiver,
}

#[bon::bon]
impl WebSocketClient {
  #[builder]
  pub async fn connect<OnEvent>(
    #[builder(start_fn)] server: ServerAddr,
    world_id: WorldId,
    world_password: Option<Password>,
    authorization: Authorization,
    on_event: OnEvent,
  ) -> Result<Self>
  where
    OnEvent: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
  {
    let (tx, rx) = match make_stream(server)
      .world_id(world_id)
      .maybe_world_password(world_password)
      .authorization(authorization)
      .call()
      .await
    {
      Ok(Either::Left(stream)) => stream.split(),
      Ok(Either::Right(TungsteniteError::Http(response))) => {
        if let Some(body) = response.into_body()
          && let Ok(body) = String::from_utf8(body)
          && !body.trim().is_empty()
        {
          return Err(Error::RequestFailed(body));
        }

        return Err(Error::FailedToConnectWebsocket);
      }
      Ok(Either::Right(_)) | Err(_) => return Err(Error::FailedToConnectWebsocket),
    };

    Ok(Self {
      _sender: Sender::new(tx),
      _receiver: Receiver::new(rx, on_event),
    })
  }
}

#[bon::builder]
async fn make_stream(
  #[builder(start_fn)] server: ServerAddr,
  world_id: WorldId,
  world_password: Option<Password>,
  authorization: Authorization,
) -> AnyResult<Either<Stream, TungsteniteError>> {
  let mut url = server.url_with_scheme("ws", "websocket")?;
  url
    .query_pairs_mut()
    .append_pair("worldId", &world_id.to_string())
    .append_pair("worldPassword", &world_password.unwrap_or_default());

  let mut request = url.into_client_request()?;
  *request.method_mut() = Method::GET;

  let headers = request.headers_mut();
  headers.insert(header::AUTHORIZATION, authorization.into_inner());
  headers.insert(header::USER_AGENT, USER_AGENT.parse()?);

  match connect_async(request).await {
    Ok((ws_stream, _)) => Ok(Either::Left(ws_stream)),
    Err(err) => Ok(Either::Right(err)),
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
}

impl Drop for Sender {
  fn drop(&mut self) {
    self.ws_sender_handle.abort();
    self.keep_alive_handle.abort();
  }
}

#[derive(Clone, Copy, Debug)]
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
  fn new<F>(mut ws_receiver: SplitStream<Stream>, on_event: F) -> Self
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
}

impl Drop for Receiver {
  fn drop(&mut self) {
    self.ws_receiver_handle.abort();
  }
}
