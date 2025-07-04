// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::chat::Chat;
use nil_core::continent::Continent;
use nil_core::lobby::Lobby;
use nil_core::player::PlayerManager;
use nil_core::round::Round;
use nil_core::script::Scripting;
use nil_core::world::World;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) struct App {
  pub(crate) world: Arc<RwLock<World>>,
  pub(crate) lobby: Arc<RwLock<Lobby>>,
}

impl App {
  pub fn new(world: World) -> Self {
    let lobby = Lobby::from(&world);
    Self {
      world: Arc::new(RwLock::new(world)),
      lobby: Arc::new(RwLock::new(lobby)),
    }
  }

  pub async fn world<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&World) -> T,
  {
    f(&*self.world.read().await)
  }

  pub async fn world_mut<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&mut World) -> T,
  {
    f(&mut *self.world.write().await)
  }

  pub async fn chat<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&Chat) -> T,
  {
    self.world(|world| f(world.chat())).await
  }

  pub async fn continent<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&Continent) -> T,
  {
    self
      .world(|world| f(world.continent()))
      .await
  }

  pub async fn lobby<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&Lobby) -> T,
  {
    f(&*self.lobby.read().await)
  }

  pub async fn lobby_mut<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&mut Lobby) -> T,
  {
    f(&mut *self.lobby.write().await)
  }

  pub async fn player_manager<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&PlayerManager) -> T,
  {
    self
      .world(|world| f(world.player_manager()))
      .await
  }

  pub async fn round<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&Round) -> T,
  {
    self.world(|world| f(world.round())).await
  }

  pub async fn scripting<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&Scripting) -> T,
  {
    self
      .world(|world| f(world.scripting()))
      .await
  }

  pub async fn scripting_mut<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&mut Scripting) -> T,
  {
    self
      .world_mut(|world| f(world.scripting_mut()))
      .await
  }
}

impl Clone for App {
  fn clone(&self) -> Self {
    Self {
      world: Arc::clone(&self.world),
      lobby: Arc::clone(&self.lobby),
    }
  }
}
