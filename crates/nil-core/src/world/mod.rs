// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod chat;
mod cheat;
mod city;
mod continent;
mod event;
mod infrastructure;
mod npc;
mod player;
mod round;
mod savedata;
mod script;

use crate::chat::Chat;
use crate::city::City;
use crate::continent::{Continent, Coord};
use crate::error::{Error, Result};
use crate::event::Emitter;
use crate::infrastructure::{Infrastructure, InfrastructureStats};
use crate::military::Military;
use crate::npc::bot::BotManager;
use crate::npc::precursor::PrecursorManager;
use crate::player::{Player, PlayerId, PlayerManager};
use crate::round::Round;
use crate::savedata::Savedata;
use crate::script::Scripting;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU8;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug)]
pub struct World {
  round: Round,
  continent: Continent,
  player_manager: PlayerManager,
  bot_manager: BotManager,
  precursor_manager: PrecursorManager,
  military: Military,
  config: WorldConfig,
  stats: WorldStats,
  chat: Chat,
  scripting: Scripting,

  // These are not included in the savedata.
  emitter: Emitter,
  pending_save: Option<PathBuf>,
}

impl World {
  pub fn new(options: &WorldOptions) -> Result<Self> {
    let config = WorldConfig::from(options);
    let continent = Continent::new(options.size.get());
    let precursor_manager = PrecursorManager::new(continent.size());
    let military = Military::new(continent.size());

    let mut world = Self {
      round: Round::default(),
      continent,
      player_manager: PlayerManager::default(),
      bot_manager: BotManager::default(),
      precursor_manager,
      military,
      config,
      stats: WorldStats::new(),
      chat: Chat::default(),
      scripting: Scripting::new(),

      emitter: Emitter::default(),
      pending_save: None,
    };

    world.spawn_precursors()?;
    world.spawn_bots()?;

    Ok(world)
  }

  #[inline]
  pub fn with_savedata(savedata: Savedata) -> Self {
    Self::from(savedata)
  }

  pub fn load(path: impl AsRef<Path>) -> Result<Self> {
    let savedata = Savedata::load(path.as_ref())?;
    Ok(Self::with_savedata(savedata))
  }

  #[inline]
  pub fn save(&mut self, path: PathBuf) {
    self.pending_save = Some(path);
  }

  #[inline]
  pub fn config(&self) -> WorldConfig {
    self.config.clone()
  }

  #[inline]
  pub fn stats(&self) -> WorldStats {
    self.stats.clone()
  }

  #[inline]
  pub fn continent(&self) -> &Continent {
    &self.continent
  }

  #[inline]
  pub fn continent_mut(&mut self) -> &mut Continent {
    &mut self.continent
  }

  #[inline]
  pub fn city(&self, coord: Coord) -> Result<&City> {
    self.continent.city(coord)
  }

  #[inline]
  pub fn city_mut(&mut self, coord: Coord) -> Result<&mut City> {
    self.continent.city_mut(coord)
  }

  #[inline]
  pub fn infrastructure(&self, coord: Coord) -> Result<&Infrastructure> {
    self.city(coord).map(City::infrastructure)
  }

  #[inline]
  pub fn infrastructure_mut(&mut self, coord: Coord) -> Result<&mut Infrastructure> {
    self
      .city_mut(coord)
      .map(City::infrastructure_mut)
  }

  #[inline]
  pub fn player_manager(&self) -> &PlayerManager {
    &self.player_manager
  }

  #[inline]
  pub fn player_manager_mut(&mut self) -> &mut PlayerManager {
    &mut self.player_manager
  }

  #[inline]
  pub fn player(&self, id: &PlayerId) -> Result<&Player> {
    self.player_manager.player(id)
  }

  #[inline]
  pub fn player_mut(&mut self, id: &PlayerId) -> Result<&mut Player> {
    self.player_manager.player_mut(id)
  }

  #[inline]
  pub fn round(&self) -> &Round {
    &self.round
  }

  #[inline]
  pub fn chat(&self) -> &Chat {
    &self.chat
  }

  #[inline]
  pub fn scripting(&self) -> &Scripting {
    &self.scripting
  }

  #[inline]
  pub fn scripting_mut(&mut self) -> &mut Scripting {
    &mut self.scripting
  }

  #[inline]
  pub fn bot_manager(&self) -> &BotManager {
    &self.bot_manager
  }

  #[inline]
  pub fn precursor_manager(&self) -> &PrecursorManager {
    &self.precursor_manager
  }
}

impl TryFrom<&WorldOptions> for World {
  type Error = Error;

  fn try_from(options: &WorldOptions) -> Result<Self> {
    Self::new(options)
  }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldOptions {
  pub name: String,
  pub size: NonZeroU8,
  pub allow_cheats: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldConfig {
  name: Arc<str>,
  allow_cheats: bool,
}

impl From<&WorldOptions> for WorldConfig {
  fn from(options: &WorldOptions) -> Self {
    Self {
      name: Arc::from(options.name.as_str()),
      allow_cheats: options.allow_cheats,
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldStats {
  infrastructure: Arc<InfrastructureStats>,
}

#[expect(clippy::new_without_default)]
impl WorldStats {
  pub fn new() -> Self {
    Self {
      infrastructure: Arc::new(InfrastructureStats::new()),
    }
  }

  #[inline]
  pub fn infrastructure(&self) -> Arc<InfrastructureStats> {
    Arc::clone(&self.infrastructure)
  }
}
