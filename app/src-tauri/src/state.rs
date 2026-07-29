// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::event::on_core_event;
use nil_client::{Client, ClientOptions};
use nil_core::world::WorldOptions;
use nil_server::local::{self, LocalServer};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Nil {
  app: AppHandle,
  client: Arc<RwLock<Client>>,
  server: Arc<RwLock<Option<LocalServer>>>,
}

impl Nil {
  pub fn new(app: &AppHandle) -> Self {
    Self {
      app: app.clone(),
      client: Arc::default(),
      server: Arc::default(),
    }
  }

  pub async fn client<F, T>(&self, f: F) -> T
  where
    F: AsyncFnOnce(&Client) -> T,
  {
    f(&*self.client.read().await).await
  }

  pub async fn update_client(&self, options: ClientOptions) -> Result<()> {
    let mut client = self.client.write().await;
    let on_event = on_core_event(self.app.clone());
    client
      .update(options, Some(on_event))
      .await?;

    Ok(())
  }

  pub async fn stop_client(&self) {
    self.client.write().await.stop().await;
  }

  async fn start_server<F>(&self, f: F) -> Result<LocalServer>
  where
    F: AsyncFnOnce() -> Result<LocalServer>,
  {
    let mut lock = self.server.write().await;
    if let Some(server) = lock.take() {
      server.stop();
    }

    let server = f().await?;
    *lock = Some(server.clone());
    Ok(server)
  }

  pub async fn start_server_with_options(&self, options: WorldOptions) -> Result<LocalServer> {
    self
      .start_server(async move || Ok(local::start(&options).await?))
      .await
  }

  pub async fn start_server_with_savedata(&self, path: PathBuf) -> Result<LocalServer> {
    self
      .start_server(async move || Ok(local::load(path).await?))
      .await
  }

  pub async fn stop_server(&self) {
    let mut lock = self.server.write().await;
    if let Some(server) = lock.take() {
      server.stop();
    }
  }

  pub async fn is_host(&self) -> bool {
    self.server.read().await.is_some()
  }

  pub async fn is_local(&self) -> bool {
    self.client.read().await.is_local()
  }

  pub async fn is_remote(&self) -> bool {
    self.client.read().await.is_remote()
  }

  pub async fn is_local_and_host(&self) -> bool {
    self.is_local().await && self.is_host().await
  }

  pub async fn is_remote_or_host(&self) -> bool {
    self.is_remote().await || self.is_host().await
  }
}
