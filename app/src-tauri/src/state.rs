// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::event::on_core_event;
use nil_client::{Client, ServerAddr};
use nil_core::player::PlayerId;
use nil_core::world::WorldOptions;
use nil_core::world::config::WorldId;
use nil_crypto::password::Password;
use nil_lua::Lua;
use nil_server::local::{self, LocalServer};
use nil_server_types::Token;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::{Mutex, RwLock};

pub struct Nil {
  app: AppHandle,
  client: Arc<RwLock<Client>>,
  server: Arc<RwLock<Option<LocalServer>>>,
  lua: Arc<Mutex<Lua>>,
}

#[bon::bon]
impl Nil {
  pub fn new(app: &AppHandle) -> Result<Self> {
    let client = Arc::default();
    let lua = Lua::new(&client)?;

    Ok(Self {
      app: app.clone(),
      client,
      server: Arc::default(),
      lua: Arc::new(Mutex::new(lua)),
    })
  }

  pub async fn client<F, T>(&self, f: F) -> T
  where
    F: AsyncFnOnce(&Client) -> T,
  {
    f(&*self.client.read().await).await
  }

  pub async fn lua<F, T>(&self, f: F) -> T
  where
    F: AsyncFnOnce(&mut Lua) -> T,
  {
    f(&mut *self.lua.lock().await).await
  }

  #[builder]
  pub async fn update_client(
    &self,
    #[builder(start_fn)] server_addr: ServerAddr,
    world_id: Option<WorldId>,
    world_password: Option<Password>,
    player_id: Option<PlayerId>,
    player_password: Option<Password>,
    authorization_token: Option<Token>,
  ) -> Result<()> {
    let mut client = self.client.write().await;
    client
      .update(server_addr)
      .maybe_world_id(world_id)
      .maybe_world_password(world_password)
      .maybe_player_id(player_id)
      .maybe_player_password(player_password)
      .maybe_authorization_token(authorization_token)
      .on_event(on_core_event(self.app.clone()))
      .call()
      .await?;

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
      .start_server(async move || Ok(local::start(&options).await?))
      .await
  }

  pub async fn start_server_with_savedata(&self, path: PathBuf) -> Result<LocalServer> {
    self
      .start_server(async move || Ok(local::load(path).await?))
      .await
  }

  pub async fn stop_client(&self) {
    self.client.write().await.stop().await;
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

impl Clone for Nil {
  fn clone(&self) -> Self {
    Nil {
      app: self.app.clone(),
      client: Arc::clone(&self.client),
      server: Arc::clone(&self.server),
      lua: Arc::clone(&self.lua),
    }
  }
}
