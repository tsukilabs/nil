// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::infrastructure::building::prefecture::PrefectureBuildOrderKind;
use crate::infrastructure::building::{BuildingId, BuildingLevel, BuildingStatsTable};
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::infrastructure::{Infrastructure, InfrastructureStats};
use crate::resources::{Maintenance, Resources, Workforce};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefectureBuildCatalog {
  academy: PrefectureBuildCatalogEntry,
  farm: PrefectureBuildCatalogEntry,
  iron_mine: PrefectureBuildCatalogEntry,
  prefecture: PrefectureBuildCatalogEntry,
  quarry: PrefectureBuildCatalogEntry,
  sawmill: PrefectureBuildCatalogEntry,
  silo: PrefectureBuildCatalogEntry,
  stable: PrefectureBuildCatalogEntry,
  wall: PrefectureBuildCatalogEntry,
  warehouse: PrefectureBuildCatalogEntry,
}

impl PrefectureBuildCatalog {
  pub fn new(infra: &Infrastructure, stats: &InfrastructureStats) -> Result<Self> {
    macro_rules! make_entry {
      ($id:ident) => {{
        let table = stats.building(BuildingId::$id)?;
        PrefectureBuildCatalogEntry::new(infra, table)
      }};
    }

    Ok(Self {
      academy: make_entry!(Academy)?,
      farm: make_entry!(Farm)?,
      iron_mine: make_entry!(IronMine)?,
      prefecture: make_entry!(Prefecture)?,
      quarry: make_entry!(Quarry)?,
      sawmill: make_entry!(Sawmill)?,
      silo: make_entry!(Silo)?,
      stable: make_entry!(Stable)?,
      wall: make_entry!(Wall)?,
      warehouse: make_entry!(Warehouse)?,
    })
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum PrefectureBuildCatalogEntry {
  /// Building is available to be built.
  Available {
    recipe: Box<PrefectureBuildCatalogRecipe>,
  },
  /// Building is already at its maximum level.
  Maxed,
  /// Village does not meet the requirements for construction.
  Unmet {
    requirements: InfrastructureRequirements,
  },
}

impl PrefectureBuildCatalogEntry {
  fn new(infrastructure: &Infrastructure, table: &BuildingStatsTable) -> Result<Self> {
    let id = table.id();
    let building = infrastructure.building(id);
    let infra_req = building.infrastructure_requirements();

    if !infra_req.has_required_levels(infrastructure) {
      return Ok(Self::Unmet { requirements: infra_req.clone() });
    }

    let target_level = infrastructure
      .prefecture
      .build_queue
      .iter()
      .filter(|order| order.building() == id)
      .fold(building.level() + 1u8, |acc, order| {
        match order.kind() {
          PrefectureBuildOrderKind::Construction => acc + 1u8,
          PrefectureBuildOrderKind::Demolition => acc - 1u8,
        }
      });

    if target_level > building.max_level() {
      Ok(Self::Maxed)
    } else {
      let stats = table.get(target_level)?;
      let recipe = Box::new(PrefectureBuildCatalogRecipe {
        level: target_level,
        resources: stats.resources.clone(),
        maintenance: stats.maintenance,
        workforce: stats.workforce,
        requirements: infra_req.clone(),
      });

      Ok(Self::Available { recipe })
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefectureBuildCatalogRecipe {
  level: BuildingLevel,
  resources: Resources,
  maintenance: Maintenance,
  workforce: Workforce,
  requirements: InfrastructureRequirements,
}
