// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::{CoreError, Error, Result};
use crate::router;
use nil_core::world::config::WorldId;
use nil_core::world::{World, WorldOptions};
use serde::Serialize;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::Path;
use tokio::sync::oneshot;
use tokio::task::{AbortHandle, spawn, spawn_blocking};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalServer {
  world: WorldId,
  addr: SocketAddrV4,

  #[serde(skip_serializing)]
  handle: AbortHandle,
}

impl LocalServer {
  async fn serve(world: World) -> Result<Self> {
    let world_id = world.config().id();
    let (tx, rx) = oneshot::channel();
    let task = spawn(async move {
      let router = router::create()
        .with_state(App::new_local(world))
        .into_make_service_with_connect_info::<SocketAddr>();

      if let Some((listener, mut addr)) = super::bind(0).await {
        if addr.ip().is_unspecified() {
          addr.set_ip(Ipv4Addr::LOCALHOST);
        }

        let _ = tx.send(Ok(addr));
        axum::serve(listener, router)
          .await
          .expect("Failed to start Call of Nil server");
      } else {
        let _ = tx.send(Err(Error::FailedToStart));
      }
    });

    Ok(Self {
      world: world_id,
      addr: rx.await.unwrap()?,
      handle: task.abort_handle(),
    })
  }

  #[inline]
  pub fn world(&self) -> WorldId {
    self.world
  }

  #[inline]
  pub fn addr(&self) -> SocketAddrV4 {
    self.addr
  }

  #[inline]
  pub fn stop(self) {
    self.handle.abort();
  }
}

pub async fn start(options: &WorldOptions) -> Result<LocalServer> {
  LocalServer::serve(options.try_into()?).await
}

pub async fn load(path: impl AsRef<Path>) -> Result<LocalServer> {
  let bytes = tokio::fs::read(path).await?;
  let world = spawn_blocking(move || World::load(&bytes))
    .await
    .map_err(|_| CoreError::FailedToReadSavedata)??;

  LocalServer::serve(world).await
}

pub(crate) fn save(path: impl AsRef<Path>, bytes: &[u8]) -> Result<()> {
  let mut path = path.as_ref().to_path_buf();
  std::fs::create_dir_all(&path)?;

  path.push(format!("{}.nil", Uuid::now_v7()));
  std::fs::write(&path, bytes)?;

  Ok(())
}
