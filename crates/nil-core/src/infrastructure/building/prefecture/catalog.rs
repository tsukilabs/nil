// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::BuildingId;
use crate::error::Result;
use crate::infrastructure::building::prefecture::PrefectureBuildOrderKind;
use crate::infrastructure::building::{BuildingLevel, BuildingStatsTable};
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::infrastructure::{Infrastructure, InfrastructureStats};
use crate::resource::{Maintenance, Resources, Workforce};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefectureCatalog {
  academy: PrefectureCatalogEntry,
  farm: PrefectureCatalogEntry,
  iron_mine: PrefectureCatalogEntry,
  prefecture: PrefectureCatalogEntry,
  quarry: PrefectureCatalogEntry,
  sawmill: PrefectureCatalogEntry,
  silo: PrefectureCatalogEntry,
  stable: PrefectureCatalogEntry,
  wall: PrefectureCatalogEntry,
  warehouse: PrefectureCatalogEntry,
}

impl PrefectureCatalog {
  pub fn new(infra: &Infrastructure, stats: &InfrastructureStats) -> Result<Self> {
    macro_rules! make_entry {
      ($id:ident) => {{
        let table = stats.building(BuildingId::$id)?;
        PrefectureCatalogEntry::new(infra, table)
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
pub enum PrefectureCatalogEntry {
  /// O edifício pode ser construído.
  Available {
    recipe: Box<PrefectureCatalogRecipe>,
  },

  /// Edifício já está no nível máximo.
  Maxed,

  /// Aldeia não atende aos requerimentos para a construção.
  Unmet {
    requirements: InfrastructureRequirements,
  },
}

impl PrefectureCatalogEntry {
  fn new(infra: &Infrastructure, table: &BuildingStatsTable) -> Result<Self> {
    let id = table.id();
    let building = infra.building(id);
    let infra_req = building.infrastructure_requirements();

    if !infra_req.has_required_levels(infra) {
      return Ok(Self::Unmet { requirements: infra_req.clone() });
    }

    let target_level = infra
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

    if target_level >= building.max_level() {
      Ok(Self::Maxed)
    } else {
      let stats = table.get(target_level)?;
      let recipe = Box::new(PrefectureCatalogRecipe {
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
pub struct PrefectureCatalogRecipe {
  level: BuildingLevel,
  resources: Resources,
  maintenance: Maintenance,
  workforce: Workforce,
  requirements: InfrastructureRequirements,
}
