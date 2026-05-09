// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Infrastructure;
use super::building::BuildingLevel;
use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[derive_const(Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[builder(const)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct InfrastructureRequirements {
  #[builder(default = BuildingLevel::ZERO)]
  pub(super) academy: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  pub(super) farm: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  pub(super) iron_mine: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  pub(super) prefecture: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  pub(super) quarry: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  pub(super) sawmill: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  pub(super) silo: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  pub(super) stable: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  pub(super) wall: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  pub(super) warehouse: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  pub(super) workshop: BuildingLevel,
}

impl InfrastructureRequirements {
  pub const fn none() -> Self {
    Self::builder().build()
  }

  /// Determines whether the city infrastructure meets the requirements.
  #[inline]
  pub fn has_required_levels(&self, infrastructure: &Infrastructure) -> bool {
    infrastructure.has_required_levels(self)
  }
}
