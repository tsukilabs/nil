// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::chat::Chat;
use nil_core::continent::Continent;
use nil_core::npc::bot::BotManager;
use nil_core::npc::precursor::PrecursorManager;
use nil_core::player::PlayerManager;
use nil_core::ranking::Ranking;
use nil_core::report::ReportManager;
use nil_core::round::Round;
use nil_core::world::World;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) struct App {
  pub(crate) world: Arc<RwLock<World>>,
}

impl App {
  pub fn new(world: World) -> Self {
    Self { world: Arc::new(RwLock::new(world)) }
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

  pub async fn bot_manager<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&BotManager) -> T,
  {
    self
      .world(|world| f(world.bot_manager()))
      .await
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

  pub async fn player_manager<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&PlayerManager) -> T,
  {
    self
      .world(|world| f(world.player_manager()))
      .await
  }

  pub async fn precursor_manager<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&PrecursorManager) -> T,
  {
    self
      .world(|world| f(world.precursor_manager()))
      .await
  }

  pub async fn ranking<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&Ranking) -> T,
  {
    self.world(|world| f(world.ranking())).await
  }

  pub async fn report_manager<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&ReportManager) -> T,
  {
    self.world(|world| f(world.report())).await
  }

  pub async fn round<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&Round) -> T,
  {
    self.world(|world| f(world.round())).await
  }
}

impl Clone for App {
  fn clone(&self) -> Self {
    Self { world: Arc::clone(&self.world) }
  }
}
