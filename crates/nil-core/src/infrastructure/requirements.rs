// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::infrastructure::Infrastructure;
use crate::infrastructure::building::{Building, BuildingLevel};
use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(const)]
pub struct InfrastructureRequirements {
  #[builder(default = BuildingLevel::ZERO)]
  academy: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  farm: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  iron_mine: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  prefecture: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  quarry: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  sawmill: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  silo: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  stable: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  wall: BuildingLevel,

  #[builder(default = BuildingLevel::ZERO)]
  warehouse: BuildingLevel,
}

impl InfrastructureRequirements {
  pub const fn none() -> Self {
    Self::builder().build()
  }

  /// Determines whether the city infrastructure meets the requirements.
  pub fn has_required_levels(&self, infrastructure: &Infrastructure) -> bool {
    infrastructure.academy.level() >= self.academy
      && infrastructure.farm.level() >= self.farm
      && infrastructure.iron_mine.level() >= self.iron_mine
      && infrastructure.prefecture.level() >= self.prefecture
      && infrastructure.quarry.level() >= self.quarry
      && infrastructure.sawmill.level() >= self.sawmill
      && infrastructure.silo.level() >= self.silo
      && infrastructure.stable.level() >= self.stable
      && infrastructure.wall.level() >= self.wall
      && infrastructure.warehouse.level() >= self.warehouse
  }
}
