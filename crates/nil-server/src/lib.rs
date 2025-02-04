#![feature(try_blocks)]

mod error;
mod response;
mod router;
mod state;
mod websocket;

pub use error::{Error, Result};
use nil_core::World;
use serde::Serialize;
use state::ServerState;
use std::net::SocketAddr;
use tauri::async_runtime::spawn;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::AbortHandle;

#[cfg(feature = "tracing")]
use tracing::{error, info};

pub struct Server {
  port: u16,
  abort_handle: AbortHandle,
}

impl Server {
  pub async fn serve(world: World) -> Result<Self> {
    let (tx, rx) = oneshot::channel();
    let task = spawn(async move {
      let router = router::create()
        .with_state(ServerState::new(world))
        .into_make_service_with_connect_info::<SocketAddr>();

      let result: Result<(TcpListener, u16)> = try {
        let listener = TcpListener::bind("0.0.0.0:0")
          .await
          .map_err(Error::FailedToBindListener)?;

        let port = listener
          .local_addr()
          .map_err(Error::FailedToGetServerPort)?
          .port();

        (listener, port)
      };

      match result {
        Ok((listener, port)) => {
          #[cfg(feature = "tracing")]
          info!("listening on port {port}");

          tx.send(Ok(port)).ok();

          axum::serve(listener, router).await.unwrap();
        }
        Err(err) => {
          #[cfg(feature = "tracing")]
          error!("failed to start server: {err}");

          tx.send(Err(err)).ok();
        }
      }
    });

    let port = rx.await.unwrap()?;
    let abort_handle = task.inner().abort_handle();

    Ok(Self { port, abort_handle })
  }

  pub fn info(&self) -> ServerInfo {
    ServerInfo { port: self.port }
  }
}

impl Drop for Server {
  fn drop(&mut self) {
    self.abort_handle.abort();
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
  pub port: u16,
}
