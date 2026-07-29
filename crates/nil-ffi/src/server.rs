// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use nil_core::world::WorldOptions;
use nil_server::local;
use nil_server::local::LocalServer;
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio::sync::RwLock;

pub(crate) static SERVER: LazyLock<RwLock<Option<LocalServer>>> = LazyLock::new(RwLock::default);

async fn start<F>(f: F) -> Result<LocalServer>
where
  F: AsyncFnOnce() -> Result<LocalServer>,
{
  let mut lock = SERVER.write().await;
  if let Some(server) = lock.take() {
    server.stop();
  }

  let server = f().await?;
  *lock = Some(server.clone());
  Ok(server)
}

pub(crate) async fn start_with_options(options: WorldOptions) -> Result<LocalServer> {
  start(async move || Ok(local::start(&options).await?)).await
}

pub(crate) async fn start_with_savedata(path: PathBuf) -> Result<LocalServer> {
  start(async move || Ok(local::load(path).await?)).await
}

pub(crate) async fn stop() {
  let mut lock = SERVER.write().await;
  if let Some(server) = lock.take() {
    server.stop();
  }
}
