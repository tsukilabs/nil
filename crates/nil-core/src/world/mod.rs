// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod chat;
mod cheat;
mod city;
mod continent;
mod event;
mod infrastructure;
mod military;
mod npc;
mod player;
mod ranking;
mod round;
mod savedata;

use crate::chat::Chat;
use crate::city::City;
use crate::continent::{Continent, Coord};
use crate::error::{Error, Result};
use crate::event::Emitter;
use crate::infrastructure::{Infrastructure, InfrastructureStats};
use crate::military::Military;
use crate::npc::bot::{Bot, BotId, BotManager};
use crate::npc::precursor::{Precursor, PrecursorId, PrecursorManager};
use crate::player::{Player, PlayerId, PlayerManager};
use crate::ranking::Ranking;
use crate::round::Round;
use crate::ruler::{Ruler, RulerRef, RulerRefMut};
use crate::savedata::Savedata;
use derive_more::From;
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
  ranking: Ranking,
  config: WorldConfig,
  stats: WorldStats,
  chat: Chat,

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
      ranking: Ranking::default(),
      config,
      stats: WorldStats::new(),
      chat: Chat::default(),

      emitter: Emitter::default(),
      pending_save: None,
    };

    world.spawn_precursors()?;
    world.spawn_bots()?;
    world.update_ranking()?;

    Ok(world)
  }

  #[inline]
  pub fn with_savedata(savedata: Savedata) -> Self {
    Self::from(savedata)
  }

  pub fn load(path: impl AsRef<Path>) -> Result<Self> {
    let savedata = Savedata::read(path.as_ref())?;
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
  pub fn round(&self) -> &Round {
    &self.round
  }

  #[inline]
  pub fn chat(&self) -> &Chat {
    &self.chat
  }

  #[inline]
  pub fn ranking(&self) -> &Ranking {
    &self.ranking
  }

  #[inline]
  pub fn player_manager(&self) -> &PlayerManager {
    &self.player_manager
  }

  #[inline]
  pub fn player(&self, id: &PlayerId) -> Result<&Player> {
    self.player_manager.player(id)
  }

  #[inline]
  fn player_mut(&mut self, id: &PlayerId) -> Result<&mut Player> {
    self.player_manager.player_mut(id)
  }

  pub fn players(&self) -> impl Iterator<Item = &Player> {
    self.player_manager.players()
  }

  #[inline]
  pub fn bot_manager(&self) -> &BotManager {
    &self.bot_manager
  }

  #[inline]
  pub fn bot(&self, id: &BotId) -> Result<&Bot> {
    self.bot_manager.bot(id)
  }

  #[inline]
  fn bot_mut(&mut self, id: &BotId) -> Result<&mut Bot> {
    self.bot_manager.bot_mut(id)
  }

  pub fn bots(&self) -> impl Iterator<Item = &Bot> {
    self.bot_manager.bots()
  }

  #[inline]
  pub fn precursor_manager(&self) -> &PrecursorManager {
    &self.precursor_manager
  }

  #[inline]
  pub fn precursor(&self, id: PrecursorId) -> &dyn Precursor {
    self.precursor_manager.precursor(id)
  }

  #[inline]
  fn precursor_mut(&mut self, id: PrecursorId) -> &mut dyn Precursor {
    self.precursor_manager.precursor_mut(id)
  }

  pub fn precursors(&self) -> impl Iterator<Item = &dyn Precursor> {
    self.precursor_manager.precursors()
  }

  pub fn ruler(&self, ruler: Ruler) -> Result<RulerRef<'_>> {
    let ruler = match ruler {
      Ruler::Bot { id } => RulerRef::Bot(self.bot(&id)?),
      Ruler::Player { id } => RulerRef::Player(self.player(&id)?),
      Ruler::Precursor { id } => RulerRef::Precursor(self.precursor(id)),
    };

    Ok(ruler)
  }

  fn ruler_mut(&mut self, ruler: Ruler) -> Result<RulerRefMut<'_>> {
    let ruler = match ruler {
      Ruler::Bot { id } => RulerRefMut::Bot(self.bot_mut(&id)?),
      Ruler::Player { id } => RulerRefMut::Player(self.player_mut(&id)?),
      Ruler::Precursor { id } => RulerRefMut::Precursor(self.precursor_mut(id)),
    };

    Ok(ruler)
  }

  pub fn rulers(&self) -> impl Iterator<Item = RulerRef<'_>> {
    self
      .players()
      .map(RulerRef::from)
      .chain(self.bots().map(RulerRef::from))
      .chain(self.precursors().map(RulerRef::from))
  }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldOptions {
  pub name: WorldName,
  pub size: NonZeroU8,
  pub allow_cheats: bool,
}

impl TryFrom<&WorldOptions> for World {
  type Error = Error;

  fn try_from(options: &WorldOptions) -> Result<Self> {
    Self::new(options)
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldConfig {
  name: WorldName,
  allow_cheats: bool,
}

impl WorldConfig {
  #[inline]
  pub fn name(&self) -> WorldName {
    self.name.clone()
  }

  #[inline]
  pub fn are_cheats_allowed(&self) -> bool {
    self.allow_cheats
  }
}

impl From<&WorldOptions> for WorldConfig {
  fn from(options: &WorldOptions) -> Self {
    Self {
      name: options.name.clone(),
      allow_cheats: options.allow_cheats,
    }
  }
}

#[derive(Clone, Debug, From, Deserialize, Serialize)]
#[from(String, &str, Arc<str>)]
pub struct WorldName(Arc<str>);

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
