#![feature(try_blocks)]

mod error;
mod response;
mod router;
mod state;
mod websocket;

pub use error::{Error, Result};
use nil_core::World;
use nil_util::spawn;
use serde::Serialize;
use state::ServerState;
use std::fmt;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::task::AbortHandle;

pub struct Server {
  addr: SocketAddr,
  abort_handle: AbortHandle,
}

impl Server {
  pub async fn serve(world: World) -> Result<Self> {
    let (tx, rx) = oneshot::channel();
    let task = spawn(async move {
      let router = router::create()
        .with_state(ServerState::new(world))
        .into_make_service_with_connect_info::<SocketAddr>();

      let result: Result<(TcpListener, SocketAddr)> = try {
        let listener = TcpListener::bind("0.0.0.0:0")
          .await
          .map_err(Error::FailedToBindListener)?;

        let addr = listener
          .local_addr()
          .map_err(Error::FailedToGetServerAddr)?;

        (listener, addr)
      };

      match result {
        Ok((listener, addr)) => {
          let _ = tx.send(Ok(addr));
          axum::serve(listener, router)
            .await
            .expect("failed to start nil server");
        }
        Err(err) => {
          tx.send(Err(err)).ok();
        }
      }
    });

    let addr = rx.await.unwrap()?;
    let abort_handle = task.abort_handle();

    Ok(Self { addr, abort_handle })
  }

  pub fn info(&self) -> ServerInfo {
    ServerInfo { port: self.addr.port() }
  }
}

impl Drop for Server {
  fn drop(&mut self) {
    self.abort_handle.abort();
  }
}

impl fmt::Debug for Server {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Server")
      .field("addr", &self.addr)
      .finish_non_exhaustive()
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
  pub port: u16,
}
