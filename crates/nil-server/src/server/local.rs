// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::{CoreError, Result};
use crate::router;
use nil_core::world::config::WorldId;
use nil_core::world::{World, WorldOptions};
use serde::Serialize;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::{Path, PathBuf};
use tokio::fs;
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
    let (listener, mut addr) = super::bind(0).await?;
    if addr.ip().is_unspecified() {
      addr.set_ip(Ipv4Addr::LOCALHOST);
    }

    let world_id = world.id();
    let router = router::create()
      .with_state(App::new_local(world))
      .into_make_service_with_connect_info::<SocketAddr>();

    let task = spawn(async move {
      axum::serve(listener, router)
        .await
        .expect("Failed to start Call of Nil server");
    });

    Ok(Self {
      world: world_id,
      addr,
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
  let bytes = fs::read(path).await?;
  let world = spawn_blocking(move || World::load(&bytes))
    .await
    .map_err(|_| CoreError::FailedToReadSavedata)??;

  LocalServer::serve(world).await
}

pub(crate) async fn save(mut dir: PathBuf, bytes: Vec<u8>) {
  let result = try {
    fs::create_dir_all(&dir).await?;
    dir.push(format!("{}.nil", Uuid::now_v7()));
    fs::write(&dir, bytes).await?;
  };

  if let Err(err) = result {
    tracing::error!(message = %err, error = ?err);
  }
}
