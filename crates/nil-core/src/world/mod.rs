// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod config;
pub mod stats;

mod battle;
mod chat;
pub mod cheat;
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
use crate::continent::Continent;
use crate::continent::size::ContinentSize;
use crate::error::{Error, Result};
use crate::event::Emitter;
use crate::hooks::OnNextRound;
use crate::market::{Market, MarketFee};
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
  market: Market,
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
  pub fn new(mut options: WorldOptions) -> Result<Self> {
    WorldOptions::clamp(&mut options);

    let config = WorldConfig::new(&options);
    let stats = WorldStats::new(&config);
    let continent = Continent::new(options.size.unwrap_or_default());
    let precursor_manager = PrecursorManager::new(continent.size());
    let military = Military::new(continent.size());
    let market = Market::new(options.market_fee.unwrap_or_default());

    let mut world = Self {
      round: Round::default(),
      continent,
      player_manager: PlayerManager::default(),
      bot_manager: BotManager::default(),
      precursor_manager,
      military,
      market,
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

  #[inline]
  pub fn market(&self) -> &Market {
    &self.market
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

impl TryFrom<WorldOptions> for World {
  type Error = Error;

  fn try_from(options: WorldOptions) -> Result<Self> {
    Self::new(options)
  }
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
pub struct WorldOptions {
  #[builder(start_fn, into)]
  pub name: WorldName,

  #[serde(default)]
  #[builder(into)]
  pub size: Option<ContinentSize>,

  #[serde(default)]
  pub locale: Option<Locale>,

  #[serde(default)]
  pub allow_cheats: Option<bool>,

  #[serde(default)]
  #[builder(into)]
  pub speed: Option<WorldSpeed>,

  #[serde(default)]
  #[builder(into)]
  pub unit_speed: Option<WorldUnitSpeed>,

  #[serde(default)]
  #[builder(into)]
  pub bot_density: Option<BotDensity>,

  #[serde(default)]
  #[builder(into)]
  pub bot_advanced_start_ratio: Option<BotAdvancedStartRatio>,

  #[serde(default)]
  #[builder(into)]
  pub market_fee: Option<MarketFee>,
}

impl WorldOptions {
  pub fn clamp(&mut self) {
    macro_rules! clamp {
      ($($field_ty:ident => $field:ident),* $(,)?) => {
        $(
          if let Some(value) = self.$field.as_mut() {
            $field_ty::clamp(value);
          }
        )*
      };
    }

    clamp!(
      ContinentSize => size,
      WorldSpeed => speed,
      WorldUnitSpeed => unit_speed,
      BotDensity => bot_density,
      BotAdvancedStartRatio => bot_advanced_start_ratio,
      MarketFee => market_fee,
    );
  }
}
