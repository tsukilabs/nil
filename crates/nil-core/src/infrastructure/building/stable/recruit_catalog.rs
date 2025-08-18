// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::infrastructure::Infrastructure;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::military::unit::{StableUnitId, Unit, UnitBox};
use crate::resources::{Maintenance, Resources, Workforce};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StableRecruitCatalog {
  heavy_cavalry: StableRecruitCatalogEntry,
  light_cavalry: StableRecruitCatalogEntry,
}

impl StableRecruitCatalog {
  pub fn new(infra: &Infrastructure) -> Self {
    macro_rules! make_entry {
      ($id:ident) => {{
        let unit = UnitBox::from(StableUnitId::$id);
        StableRecruitCatalogEntry::new(unit.as_dyn(), infra)
      }};
    }

    Self {
      heavy_cavalry: make_entry!(HeavyCavalry),
      light_cavalry: make_entry!(LightCavalry),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum StableRecruitCatalogEntry {
  /// Unit is available for recruitment.
  Available {
    recipe: Box<StableRecruitCatalogRecipe>,
  },
  /// City does not meet the requirements for recruitment.
  Unmet {
    requirements: InfrastructureRequirements,
  },
}

impl StableRecruitCatalogEntry {
  fn new(unit: &dyn Unit, infrastructure: &Infrastructure) -> Self {
    let infra_req = unit.infrastructure_requirements();
    if !infra_req.has_required_levels(infrastructure) {
      return Self::Unmet { requirements: infra_req.clone() };
    }

    let chunk = unit.chunk();
    let recipe = Box::new(StableRecruitCatalogRecipe {
      resources: chunk.resources(),
      maintenance: chunk.maintenance(),
      workforce: chunk.workforce(),
      requirements: infra_req.clone(),
    });

    Self::Available { recipe }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StableRecruitCatalogRecipe {
  resources: Resources,
  maintenance: Maintenance,
  workforce: Workforce,
  requirements: InfrastructureRequirements,
}
