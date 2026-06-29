// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod config;
pub mod stats;

mod battle;
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
mod report;
mod resources;
mod round;
mod savedata;

use crate::chat::Chat;
use crate::continent::{Continent, ContinentSize};
use crate::error::{Error, Result};
use crate::event::Emitter;
use crate::hooks::OnNextRound;
use crate::military::Military;
use crate::npc::bot::BotManager;
use crate::npc::precursor::PrecursorManager;
use crate::player::PlayerManager;
use crate::ranking::Ranking;
use crate::round::Round;
use crate::ruler::{Ruler, RulerRef, RulerRefMut};
use crate::savedata::{SaveHandle, Savedata};
use crate::world::config::{WorldSpeed, WorldUnitSpeed};
use bon::Builder;
use config::{BotAdvancedStartRatio, BotDensity, Locale, WorldConfig, WorldId, WorldName};
use serde::{Deserialize, Serialize};
use stats::WorldStats;
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
  chat: Chat,

  config: Arc<WorldConfig>,
  stats: WorldStats,

  // These are not included in the savedata.
  emitter: Emitter,
  save_handle: Option<SaveHandle>,
  on_next_round: Option<OnNextRound>,
}

impl World {
  pub fn new(options: &WorldOptions) -> Result<Self> {
    let config = WorldConfig::new(options);
    let stats = WorldStats::new(&config);
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
      config: Arc::new(config),
      stats,
      chat: Chat::default(),

      emitter: Emitter::default(),
      save_handle: None,
      on_next_round: None,
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

  pub fn load(bytes: &[u8]) -> Result<Self> {
    let savedata = Savedata::read(bytes)?;
    Ok(Self::with_savedata(savedata))
  }

  #[inline]
  pub fn id(&self) -> WorldId {
    self.config.id()
  }

  #[inline]
  pub fn config(&self) -> Arc<WorldConfig> {
    Arc::clone(&self.config)
  }

  #[inline]
  pub fn stats(&self) -> WorldStats {
    self.stats.clone()
  }

  pub fn ruler(&self, ruler: &Ruler) -> Result<RulerRef<'_>> {
    let ruler = match ruler {
      Ruler::Bot { id } => RulerRef::Bot(self.bot(id)?),
      Ruler::Player { id } => RulerRef::Player(self.player(id)?),
      Ruler::Precursor { id } => RulerRef::Precursor(self.precursor(*id)),
    };

    Ok(ruler)
  }

  fn ruler_mut(&mut self, ruler: &Ruler) -> Result<RulerRefMut<'_>> {
    let ruler = match ruler {
      Ruler::Bot { id } => RulerRefMut::Bot(self.bot_mut(id)?),
      Ruler::Player { id } => RulerRefMut::Player(self.player_mut(id)?),
      Ruler::Precursor { id } => RulerRefMut::Precursor(self.precursor_mut(*id)),
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

  #[inline]
  pub fn military(&self) -> &Military {
    &self.military
  }

  /// Schedules a save to be performed at the end of the current round.
  /// If a save is already scheduled, it will be overwritten.
  pub fn save<F>(&mut self, f: F)
  where
    F: FnOnce(Vec<u8>) + Send + Sync + 'static,
  {
    self.save_handle = Some(SaveHandle::new(f));
  }

  /// Registers a hook to be called once a new round is about to start.
  pub fn on_next_round<F>(&mut self, f: F)
  where
    F: Fn(&mut World) + Send + Sync + 'static,
  {
    self.on_next_round = Some(OnNextRound::new(f));
  }
}

impl Drop for World {
  fn drop(&mut self) {
    let _ = self.emit_drop();
  }
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct WorldOptions {
  #[builder(start_fn, into)]
  pub name: WorldName,

  #[serde(default)]
  #[builder(default, into)]
  pub size: ContinentSize,

  #[serde(default)]
  #[builder(default)]
  pub locale: Locale,

  #[serde(default)]
  #[builder(default)]
  pub allow_cheats: bool,

  #[serde(default)]
  #[builder(default, into)]
  pub speed: WorldSpeed,

  #[serde(default)]
  #[builder(default, into)]
  pub unit_speed: WorldUnitSpeed,

  #[serde(default)]
  #[builder(default, into)]
  pub bot_density: BotDensity,

  #[serde(default)]
  #[builder(default, into)]
  pub bot_advanced_start_ratio: BotAdvancedStartRatio,
}

impl WorldOptions {
  pub fn to_world(&self) -> Result<World> {
    World::try_from(self)
  }
}

impl TryFrom<&WorldOptions> for World {
  type Error = Error;

  fn try_from(options: &WorldOptions) -> Result<Self> {
    Self::new(options)
  }
}
