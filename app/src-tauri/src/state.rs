// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use futures::future::BoxFuture;
use nil_client::{Client, ServerAddr};
use nil_core::event::Event;
use nil_core::player::PlayerId;
use nil_core::world::{WorldId, WorldOptions};
use nil_server::local::{LocalServer, load_local, start_local};
use nil_server_types::Password;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

pub type NilClient = Arc<RwLock<Option<Client>>>;
pub type NilServer = Arc<RwLock<Option<LocalServer>>>;

pub struct Nil {
  app: AppHandle,
  client: NilClient,
  server: NilServer,
}

impl Nil {
  pub fn new(app: &AppHandle) -> Self {
    Self {
      app: app.clone(),
      client: Arc::new(RwLock::new(None)),
      server: Arc::new(RwLock::new(None)),
    }
  }

  pub async fn client<F, T>(&self, f: F) -> Result<T>
  where
    F: AsyncFnOnce(&Client) -> T,
  {
    let client = &self.client;
    match client.read().await.as_ref() {
      Some(client) => Ok(f(client).await),
      None => Err(Error::ClientNotConnected),
    }
  }

  pub async fn is_host(&self) -> bool {
    self.server.read().await.is_some()
  }

  pub async fn start_client(
    &self,
    server_addr: ServerAddr,
    world_id: Option<WorldId>,
    player_id: PlayerId,
    password: Option<Password>,
  ) -> Result<()> {
    let mut lock = self.client.write().await;
    *lock = None;

    let client = Client::start()
      .server(server_addr)
      .maybe_world_id(world_id)
      .player_id(player_id)
      .maybe_password(password)
      .on_event(on_event(self.app.clone()))
      .call()
      .await?;

    *lock = Some(client);

    Ok(())
  }

  async fn start_server<F>(&self, f: F) -> Result<LocalServer>
  where
    F: AsyncFnOnce() -> Result<LocalServer>,
  {
    let mut lock = self.server.write().await;
    *lock = None;

    let server = f().await?;
    *lock = Some(server.clone());
    Ok(server)
  }

  pub async fn start_server_with_options(&self, options: WorldOptions) -> Result<LocalServer> {
    self
      .start_server(async move || Ok(start_local(&options).await?))
      .await
  }

  pub async fn start_server_with_savedata(&self, path: PathBuf) -> Result<LocalServer> {
    self
      .start_server(async move || Ok(load_local(path).await?))
      .await
  }

  pub async fn stop_client(&self) {
    let mut lock = self.client.write().await;
    if let Some(client) = lock.take() {
      client.stop().await;
    }
  }

  pub async fn stop_server(&self) {
    let mut lock = self.server.write().await;
    if let Some(server) = lock.take() {
      server.stop();
    }
  }
}

impl Clone for Nil {
  fn clone(&self) -> Self {
    Nil {
      app: self.app.clone(),
      client: Arc::clone(&self.client),
      server: Arc::clone(&self.server),
    }
  }
}

fn on_event(app: AppHandle) -> impl Fn(Event) -> BoxFuture<'static, ()> {
  move |event: Event| {
    let app = app.clone();
    Box::pin(async move {
      let name = format!("nil://{event}");
      let _ = app.emit_to("main", &name, event);
    })
  }
}
