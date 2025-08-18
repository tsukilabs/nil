// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::city::CityOwner;
use crate::military::squad::{Squad, SquadSize};
use crate::military::unit::UnitId;
use crate::npc::bot::BotId;
use crate::npc::precursor::PrecursorId;
use crate::player::PlayerId;
use bon::Builder;
use nil_core_macros::Owner;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use strum::EnumIs;
use uuid::Uuid;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[builder(builder_type(vis = "pub(in crate::military)"))]
pub struct Army {
  #[builder(skip)]
  id: ArmyId,

  #[builder(default)]
  personnel: ArmyPersonnel,

  #[builder(into)]
  owner: ArmyOwner,

  #[builder(skip)]
  state: ArmyState,
}

impl Army {
  #[inline]
  pub fn personnel(&self) -> &ArmyPersonnel {
    &self.personnel
  }

  #[inline]
  pub fn owner(&self) -> &ArmyOwner {
    &self.owner
  }

  #[inline]
  pub fn state(&self) -> &ArmyState {
    &self.state
  }

  #[inline]
  pub fn is_idle(&self) -> bool {
    self.state.is_idle()
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ArmyId(Uuid);

impl ArmyId {
  #[must_use]
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Default for ArmyId {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmyPersonnel {
  archer: Squad,
  axeman: Squad,
  heavy_cavalry: Squad,
  light_cavalry: Squad,
  pikeman: Squad,
  swordsman: Squad,
}

#[bon::bon]
impl ArmyPersonnel {
  #[builder]
  pub fn new(
    #[builder(default, into)] archer: SquadSize,
    #[builder(default, into)] axeman: SquadSize,
    #[builder(default, into)] heavy_cavalry: SquadSize,
    #[builder(default, into)] light_cavalry: SquadSize,
    #[builder(default, into)] pikeman: SquadSize,
    #[builder(default, into)] swordsman: SquadSize,
  ) -> Self {
    use UnitId::*;
    Self {
      archer: Squad::new(Archer, archer),
      axeman: Squad::new(Axeman, axeman),
      heavy_cavalry: Squad::new(HeavyCavalry, heavy_cavalry),
      light_cavalry: Squad::new(LightCavalry, light_cavalry),
      pikeman: Squad::new(Pikeman, pikeman),
      swordsman: Squad::new(Swordsman, swordsman),
    }
  }
}

impl Default for ArmyPersonnel {
  fn default() -> Self {
    Self::builder().build()
  }
}

impl FromIterator<Squad> for ArmyPersonnel {
  fn from_iter<T>(iter: T) -> Self
  where
    T: IntoIterator<Item = Squad>,
  {
    iter
      .into_iter()
      .fold(Self::default(), |mut personnel, squad| {
        personnel += squad;
        personnel
      })
  }
}

impl Add for ArmyPersonnel {
  type Output = ArmyPersonnel;

  fn add(mut self, rhs: Self) -> Self::Output {
    self += rhs;
    self
  }
}

impl Add<Squad> for ArmyPersonnel {
  type Output = ArmyPersonnel;

  fn add(mut self, rhs: Squad) -> Self::Output {
    self += rhs;
    self
  }
}

impl AddAssign for ArmyPersonnel {
  fn add_assign(&mut self, rhs: Self) {
    self.archer += rhs.archer;
    self.axeman += rhs.axeman;
    self.heavy_cavalry += rhs.heavy_cavalry;
    self.light_cavalry += rhs.light_cavalry;
    self.pikeman += rhs.pikeman;
    self.swordsman += rhs.swordsman;
  }
}

impl AddAssign<Squad> for ArmyPersonnel {
  fn add_assign(&mut self, rhs: Squad) {
    match rhs.id() {
      UnitId::Archer => self.archer += rhs,
      UnitId::Axeman => self.axeman += rhs,
      UnitId::HeavyCavalry => self.heavy_cavalry += rhs,
      UnitId::LightCavalry => self.light_cavalry += rhs,
      UnitId::Pikeman => self.pikeman += rhs,
      UnitId::Swordsman => self.swordsman += rhs,
    }
  }
}

impl Sub for ArmyPersonnel {
  type Output = ArmyPersonnel;

  fn sub(mut self, rhs: Self) -> Self::Output {
    self -= rhs;
    self
  }
}

impl SubAssign for ArmyPersonnel {
  fn sub_assign(&mut self, rhs: Self) {
    self.archer -= rhs.archer;
    self.axeman -= rhs.axeman;
    self.heavy_cavalry -= rhs.heavy_cavalry;
    self.light_cavalry -= rhs.light_cavalry;
    self.pikeman -= rhs.pikeman;
    self.swordsman -= rhs.swordsman;
  }
}

impl SubAssign<Squad> for ArmyPersonnel {
  fn sub_assign(&mut self, rhs: Squad) {
    match rhs.id() {
      UnitId::Archer => self.archer -= rhs,
      UnitId::Axeman => self.axeman -= rhs,
      UnitId::HeavyCavalry => self.heavy_cavalry -= rhs,
      UnitId::LightCavalry => self.light_cavalry -= rhs,
      UnitId::Pikeman => self.pikeman -= rhs,
      UnitId::Swordsman => self.swordsman -= rhs,
    }
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ArmyState {
  #[default]
  Idle,
}

#[allow(variant_size_differences)]
#[derive(Owner, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ArmyOwner {
  Bot { id: BotId },
  Player { id: PlayerId },
  Precursor { id: PrecursorId },
}

impl From<&CityOwner> for ArmyOwner {
  fn from(owner: &CityOwner) -> Self {
    match owner.clone() {
      CityOwner::Bot { id } => Self::Bot { id },
      CityOwner::Player { id } => Self::Player { id },
      CityOwner::Precursor { id } => Self::Precursor { id },
    }
  }
}
