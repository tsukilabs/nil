// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::authorization::Authorization;
use crate::circuit_breaker::{CircuitBreaker, CircuitState};
use crate::error::{AnyResult, Error, Result};
use crate::http::has_html_content_type;
use crate::retry::{Retry, is_retryable_error};
use crate::server::ServerAddr;
use anyhow::anyhow;
use bytes::Bytes;
use futures::future::BoxFuture;
use futures::stream::{SplitSink, SplitStream};
use futures::{Sink, SinkExt, StreamExt};
use http::{Method, Request, Response, header};
use nil_core::event::Event;
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use std::num::NonZeroU8;
use std::ops::ControlFlow;
use std::sync::Weak;
use std::sync::nonpoison::Mutex;
use tap::TapFallible;
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
    circuit_breaker: Weak<Mutex<CircuitBreaker>>,
    user_agent: &str,
    on_event: OnEvent,
  ) -> Result<Self>
  where
    OnEvent: Fn(Event) -> BoxFuture<'static, ()> + Send + Sync + 'static,
  {
    let retry = Retry::builder()
      .attempts(unsafe { NonZeroU8::new_unchecked(3) })
      .backoff(false)
      .build();

    for attempt in 1..=retry.attempts() {
      if let Some(circuit_breaker) = Weak::upgrade(&circuit_breaker)
        && let CircuitState::Open = circuit_breaker.lock().update()
      {
        return Err(Error::ServiceUnavailable);
      }

      let (tx, rx) = match make_stream(server)
        .world_id(world_id)
        .maybe_world_password(world_password.clone())
        .authorization(authorization.clone())
        .user_agent(user_agent)
        .call()
        .await
      {
        Ok(stream) => stream.split(),
        Err(err) => {
          tracing::error!(message = %err, error = ?err);
          if let Some(circuit_breaker) = Weak::upgrade(&circuit_breaker) {
            circuit_breaker.lock().record_failure();
          }

          if attempt < retry.attempts() && is_retryable_error(&err) {
            sleep(retry.delay(attempt)).await;
            continue;
          }

          if let Error::Tungstenite(TungsteniteError::Http(response)) = err {
            return Err(handle_http_error(*response));
          } else {
            return Err(Error::FailedToConnectWebsocket(None));
          }
        }
      };

      if let Some(circuit_breaker) = Weak::upgrade(&circuit_breaker) {
        circuit_breaker.lock().record_success();
      }

      return Ok(Self {
        _sender: Sender::new(tx),
        _receiver: Receiver::new(rx, on_event),
      });
    }

    unreachable!();
  }
}

#[bon::builder]
async fn make_stream(
  #[builder(start_fn)] server: ServerAddr,
  world_id: WorldId,
  world_password: Option<Password>,
  authorization: Authorization,
  user_agent: &str,
) -> Result<Stream> {
  // Can't use try blocks with `bon`.
  let make_request = || -> AnyResult<Request<()>> {
    let mut url = server.url("websocket")?;
    if url.scheme().eq_ignore_ascii_case("https") {
      let _ = url.set_scheme("wss");
    } else {
      let _ = url.set_scheme("ws");
    }

    url
      .query_pairs_mut()
      .append_pair("worldId", &world_id.to_string())
      .append_pair("worldPassword", &world_password.unwrap_or_default());

    let mut request = url.into_client_request()?;
    *request.method_mut() = Method::GET;

    let headers = request.headers_mut();
    headers.insert(header::AUTHORIZATION, authorization.into_inner());
    headers.insert(header::USER_AGENT, user_agent.parse()?);

    #[cfg(debug_assertions)]
    tracing::debug!(?request);

    Ok(request)
  };

  connect_async(make_request()?)
    .await
    .tap_ok_dbg(|(_, response)| tracing::debug!(?response))
    .map(|(stream, _)| stream)
    .map_err(Into::into)
}

fn handle_http_error(response: Response<Option<Vec<u8>>>) -> Error {
  if has_html_content_type(response.headers()) {
    let status = response.status();
    if let Some(reason) = status.canonical_reason() {
      return Error::FailedToConnectWebsocket(Some(reason.to_owned()));
    } else {
      return Error::Unknown(anyhow!("Unknown websocket error: {status}"));
    }
  }

  if let Some(body) = response.into_body()
    && let Ok(body) = String::from_utf8(body)
    && !body.trim().is_empty()
  {
    return Error::FailedToConnectWebsocket(Some(body));
  }

  Error::FailedToConnectWebsocket(None)
}

struct Sender {
  ws_sender_handle: AbortHandle,
  keep_alive_handle: AbortHandle,
}

impl Sender {
  fn new<T>(mut ws_sender: SplitSink<T, Message>) -> Self
  where
    T: Sink<Message> + Send + 'static,
    T::Error: Into<TungsteniteError>,
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
    T::Error: Into<TungsteniteError>,
  {
    match self {
      SenderMessage::KeepAlive => {
        if let Err(err) = ws_sender
          .send(Message::Ping(Bytes::default()))
          .await
          .map_err(Into::<TungsteniteError>::into)
        {
          tracing::error!(message = %err, error = ?err);

          if matches!(
            err,
            TungsteniteError::AlreadyClosed | TungsteniteError::ConnectionClosed
          ) {
            return ControlFlow::Break(());
          }
        }
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
      while let Some(result) = ws_receiver.next().await {
        match result {
          Ok(Message::Binary(bytes)) => on_event(Event::from(bytes)).await,
          Ok(Message::Close(_)) => break,
          Err(err) => {
            tracing::error!(message = %err, error = ?err);
            break;
          }
          _ => {}
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
