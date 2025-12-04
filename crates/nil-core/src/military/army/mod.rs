// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod personnel;

use std::mem;

use crate::military::maneuver::ManeuverId;
use crate::military::unit::stats::speed::Speed;
use crate::ranking::Score;
use crate::resources::Maintenance;
use crate::ruler::Ruler;
use bon::Builder;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use strum::EnumIs;
use uuid::Uuid;

pub use personnel::ArmyPersonnel;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[builder(builder_type(vis = "pub(in crate::military)"))]
#[serde(rename_all = "camelCase")]
pub struct Army {
  #[builder(skip)]
  id: ArmyId,

  #[builder(default)]
  personnel: ArmyPersonnel,

  #[builder(into)]
  owner: Ruler,

  #[builder(default, into)]
  state: ArmyState,
}

impl Army {
  #[inline]
  pub fn id(&self) -> ArmyId {
    self.id
  }

  #[inline]
  pub fn personnel(&self) -> &ArmyPersonnel {
    &self.personnel
  }

  pub(crate) fn personnel_mut(&mut self) -> &mut ArmyPersonnel {
    &mut self.personnel
  }

  #[inline]
  pub fn owner(&self) -> &Ruler {
    &self.owner
  }

  #[inline]
  pub fn state(&self) -> &ArmyState {
    &self.state
  }

  pub(crate) fn state_mut(&mut self) -> &mut ArmyState {
    &mut self.state
  }

  #[inline]
  pub fn speed(&self) -> Option<Speed> {
    self.personnel.speed()
  }

  #[inline]
  pub fn score(&self) -> Score {
    self.personnel.score()
  }

  #[inline]
  pub fn maintenance(&self) -> Maintenance {
    self.personnel.maintenance()
  }

  #[inline]
  pub fn is_owned_by(&self, ruler: &Ruler) -> bool {
    self.owner.eq(ruler)
  }

  #[inline]
  pub fn is_idle_and_owned_by(&self, ruler: &Ruler) -> bool {
    self.is_idle() && self.is_owned_by(ruler)
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.personnel.is_empty()
  }

  #[inline]
  pub fn is_idle(&self) -> bool {
    self.state.is_idle()
  }

  #[inline]
  pub fn is_maneuvering(&self) -> bool {
    self.state.is_maneuvering()
  }

  #[inline]
  pub fn has_enough_personnel(&self, required: &ArmyPersonnel) -> bool {
    self.personnel.has_enough_personnel(required)
  }
}

impl From<Army> for ArmyPersonnel {
  fn from(army: Army) -> Self {
    army.personnel
  }
}

#[must_use]
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Deserialize, Serialize)]
pub struct ArmyId(Uuid);

impl ArmyId {
  #[inline]
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Default for ArmyId {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ArmyState {
  #[default]
  Idle,
  Maneuvering {
    maneuver: ManeuverId,
  },
}

impl From<ManeuverId> for ArmyState {
  fn from(maneuver: ManeuverId) -> Self {
    Self::Maneuvering { maneuver }
  }
}

pub fn find_idle_owned_by<'a>(armies: &'a mut [Army], ruler: &Ruler) -> Option<&'a mut Army> {
  armies
    .iter_mut()
    .find(|army| army.is_idle_and_owned_by(ruler))
}

pub fn collapse_armies(armies: &mut Vec<Army>) {
  for army in mem::take(armies) {
    if army.is_idle()
      && let Some(previous) = find_idle_owned_by(armies, army.owner())
    {
      *previous.personnel_mut() += ArmyPersonnel::from(army);
    } else {
      armies.push(army);
    }
  }
}
