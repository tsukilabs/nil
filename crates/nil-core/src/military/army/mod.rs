// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod personnel;

use crate::ranking::Score;
use crate::resources::Maintenance;
use crate::ruler::Ruler;
use bon::Builder;
use serde::{Deserialize, Serialize};
use strum::EnumIs;
use uuid::Uuid;

pub use personnel::ArmyPersonnel;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[builder(builder_type(vis = "pub(in crate::military)"))]
pub struct Army {
  #[builder(skip)]
  id: ArmyId,

  #[builder(default)]
  personnel: ArmyPersonnel,

  #[builder(into)]
  owner: Ruler,

  #[builder(skip)]
  state: ArmyState,
}

impl Army {
  #[inline]
  pub fn personnel(&self) -> &ArmyPersonnel {
    &self.personnel
  }

  #[inline]
  pub fn owner(&self) -> &Ruler {
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

  #[inline]
  pub fn score(&self) -> Score {
    self.personnel.score()
  }

  #[inline]
  pub fn maintenance(&self) -> Maintenance {
    self.personnel.maintenance()
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, EnumIs)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ArmyState {
  #[default]
  Idle,
}
