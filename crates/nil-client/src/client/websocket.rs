use super::USER_AGENT;
use crate::error::{Error, Result};
use anyhow::Result as AnyResult;
use bytes::Bytes;
use futures::stream::{SplitSink, SplitStream};
use futures::{Sink, SinkExt, StreamExt};
use http::header::USER_AGENT as USER_AGENT_HEADER;
use nil_core::Event;
use nil_util::spawn;
use std::fmt;
use std::net::SocketAddrV4;
use std::ops::{ControlFlow, Not};
use tokio::net::TcpStream;
use tokio::sync::mpsc::channel;
use tokio::task::AbortHandle;
use tokio::time::{Duration, sleep};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

#[cfg(feature = "tracing")]
use tracing::instrument;

type ReceiverStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

pub(super) struct WebSocketClient {
  sender: Sender,
  receiver: Receiver,
}

impl WebSocketClient {
  #[cfg_attr(feature = "tracing", instrument(skip(on_event), err, ret))]
  pub async fn connect<F>(addr: &SocketAddrV4, on_event: F) -> Result<Self>
  where
    F: Fn(Event) + Send + Sync + 'static,
  {
    let ws_stream: AnyResult<WebSocketStream<_>> = try {
      let url = format!("ws://{addr}/ws");
      let mut request = url.into_client_request()?;
      request
        .headers_mut()
        .insert(USER_AGENT_HEADER, USER_AGENT.parse()?);

      connect_async(request)
        .await
        .map(|(ws_stream, _)| ws_stream)?
    };

    let Ok(ws_stream) = ws_stream else {
      return Err(Error::FailedToConnectWebsocket {
        reason: ws_stream.unwrap_err().to_string(),
      });
    };

    let (tx, rx) = ws_stream.split();

    Ok(Self {
      sender: Sender::new(tx),
      receiver: Receiver::new(rx, on_event),
    })
  }
}

impl fmt::Debug for WebSocketClient {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("WebSocketClient")
      .field("sender", &self.sender)
      .field("receiver", &self.receiver)
      .finish_non_exhaustive()
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
      while tx
        .send(SenderMessage::KeepAlive)
        .await
        .is_ok()
      {
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
    self.keep_alive_handle.abort();
    self.ws_sender_handle.abort();
  }
}

impl fmt::Debug for Sender {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Sender")
      .field(
        "ws_sender_handle",
        &self.ws_sender_handle.is_finished().not(),
      )
      .field(
        "keep_alive_handle",
        &self.keep_alive_handle.is_finished().not(),
      )
      .finish()
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
    F: Fn(Event) + Send + Sync + 'static,
  {
    let ws_receiver_task = spawn(async move {
      while let Some(Ok(message)) = ws_receiver.next().await {
        if let Message::Binary(bytes) = message {
          if let Ok(event) = Event::try_from(bytes) {
            on_event(event);
          }
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

impl fmt::Debug for Receiver {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Receiver")
      .field(
        "ws_receiver_handle",
        &self.ws_receiver_handle.is_finished().not(),
      )
      .finish()
  }
}
