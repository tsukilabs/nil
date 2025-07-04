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

  /// Determina se a infraestrutura da aldeia atende aos requerimentos.
  pub fn has_required_levels(&self, infra: &Infrastructure) -> bool {
    infra.academy.level() >= self.academy
      && infra.farm.level() >= self.farm
      && infra.iron_mine.level() >= self.iron_mine
      && infra.prefecture.level() >= self.prefecture
      && infra.quarry.level() >= self.quarry
      && infra.sawmill.level() >= self.sawmill
      && infra.silo.level() >= self.silo
      && infra.stable.level() >= self.stable
      && infra.wall.level() >= self.wall
      && infra.warehouse.level() >= self.warehouse
  }
}
