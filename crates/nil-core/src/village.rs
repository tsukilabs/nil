// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::infrastructure::{Infrastructure, InfrastructureStats};
use crate::player::PlayerId;
use crate::resource::{Maintenance, Resources};
use bon::Builder;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Village {
  #[builder(start_fn, into)]
  coord: Coord,

  #[builder(into)]
  name: String,

  #[builder(into)]
  owner: VillageOwner,

  #[builder(default)]
  infrastructure: Infrastructure,

  #[builder(default)]
  stability: Stability,
}

impl Village {
  #[inline]
  pub fn coord(&self) -> Coord {
    self.coord
  }

  #[inline]
  pub fn name(&self) -> &str {
    &self.name
  }

  #[inline]
  pub fn owner(&self) -> &VillageOwner {
    &self.owner
  }

  #[inline]
  pub fn infrastructure(&self) -> &Infrastructure {
    &self.infrastructure
  }

  #[inline]
  pub fn infrastructure_mut(&mut self) -> &mut Infrastructure {
    &mut self.infrastructure
  }

  #[inline]
  pub fn stability(&self) -> Stability {
    self.stability
  }

  #[inline]
  pub(crate) fn stability_mut(&mut self) -> &mut Stability {
    &mut self.stability
  }

  #[inline]
  pub fn player(&self) -> Option<PlayerId> {
    self.owner().player().cloned()
  }

  /// Indica se a aldeia pertence a algum jogador.
  #[inline]
  pub fn is_owned_by_player(&self) -> bool {
    self.owner.player().is_some()
  }

  pub fn is_owned_by_player_and<F>(&self, f: F) -> bool
  where
    F: FnOnce(&PlayerId) -> bool,
  {
    self.owner.player().is_some_and(f)
  }

  /// Determina a quantidade de recursos gerados pelas minas da aldeia,
  /// aplicando todos os modificadores relevantes, como, por exemplo, a estabilidade atual.
  pub fn round_production(&self, stats: &InfrastructureStats) -> Result<Resources> {
    let mut resources = self
      .infrastructure
      .round_base_production(stats)?;

    resources.food *= self.stability;
    resources.iron *= self.stability;
    resources.stone *= self.stability;
    resources.wood *= self.stability;

    Ok(resources)
  }

  /// Determina a taxa de manutenção exigida pelos edifícios da aldeia.
  ///
  /// No momento, não há modificadores para a taxa, mas, quanto houver, eles deverão ser aplicados aqui.
  pub fn round_maintenance(&self, stats: &InfrastructureStats) -> Result<Maintenance> {
    self
      .infrastructure
      .round_base_maintenance(stats)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Coord {
  x: u8,
  y: u8,
}

impl Coord {
  #[inline]
  pub const fn new(x: u8, y: u8) -> Self {
    Self { x, y }
  }

  #[inline]
  pub const fn x(&self) -> u8 {
    self.x
  }

  #[inline]
  pub const fn y(&self) -> u8 {
    self.y
  }
}

impl From<(u8, u8)> for Coord {
  fn from((x, y): (u8, u8)) -> Self {
    Self::new(x, y)
  }
}

impl fmt::Display for Coord {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:03}|{:03}", self.x, self.y)
  }
}

/// Representa a estabilidade política da aldeia.
#[derive(Clone, Copy, Debug, Deref, Deserialize, Serialize)]
pub struct Stability(f64);

impl Stability {
  pub const MIN: Stability = Stability(0.0);
  pub const MAX: Stability = Stability(1.0);

  #[inline]
  pub const fn new(value: f64) -> Self {
    Self(value.clamp(Self::MIN.0, Self::MAX.0))
  }
}

impl Default for Stability {
  fn default() -> Self {
    Self::MAX
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum VillageOwner {
  #[default]
  None,
  Player {
    id: PlayerId,
  },
}

impl VillageOwner {
  /// Obtém o id do jogado dono da aldeia, se houver algum.
  #[inline]
  pub fn player(&self) -> Option<&PlayerId> {
    if let Self::Player { id } = self {
      Some(id)
    } else {
      None
    }
  }
}

impl From<PlayerId> for VillageOwner {
  fn from(id: PlayerId) -> Self {
    Self::Player { id }
  }
}

impl From<&PlayerId> for VillageOwner {
  fn from(id: &PlayerId) -> Self {
    Self::Player { id: id.clone() }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicVillage {
  coord: Coord,
  name: String,
  owner: VillageOwner,
}

impl From<&Village> for PublicVillage {
  fn from(village: &Village) -> Self {
    Self {
      coord: village.coord,
      name: village.name.clone(),
      owner: village.owner.clone(),
    }
  }
}
