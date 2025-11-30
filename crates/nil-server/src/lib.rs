// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(try_blocks)]

mod error;
mod middleware;
mod response;
mod router;
mod state;
mod websocket;

use error::{AnyResult, Result};
use nil_core::world::{World, WorldOptions};
use state::App;
use std::net::{SocketAddr, SocketAddrV4};
use std::path::Path;
use tokio::net::TcpListener;
use tokio::spawn;
use tokio::sync::oneshot;
use tokio::task::AbortHandle;

pub use error::Error;

pub struct Server(AbortHandle);

impl Server {
  async fn serve(world: World) -> Result<(Self, SocketAddrV4)> {
    let (tx, rx) = oneshot::channel();
    let task = spawn(async move {
      let router = router::create()
        .with_state(App::new(world))
        .into_make_service_with_connect_info::<SocketAddr>();

      if let Some((listener, addr)) = bind().await {
        let _ = tx.send(Ok(addr));
        axum::serve(listener, router)
          .await
          .expect("failed to start nil server");
      } else {
        let _ = tx.send(Err(Error::FailedToStart));
      }
    });

    let addr = rx.await.unwrap()?;
    let server = Server(task.abort_handle());
    Ok((server, addr))
  }

  pub fn stop(self) {
    self.0.abort();
  }
}

pub async fn new_game(options: &WorldOptions) -> Result<(Server, SocketAddrV4)> {
  Server::serve(options.try_into()?).await
}

/// Inicia a partir de um jogo previamente salvo.
pub async fn load_game(path: impl AsRef<Path>) -> Result<(Server, SocketAddrV4)> {
  Server::serve(World::load(path)?).await
}

async fn bind() -> Option<(TcpListener, SocketAddrV4)> {
  let result: AnyResult<(TcpListener, SocketAddrV4)> = try {
    let listener = TcpListener::bind("0.0.0.0:0").await?;
    let SocketAddr::V4(addr) = listener.local_addr()? else {
      unreachable!();
    };

    (listener, addr)
  };

  result.ok()
}
