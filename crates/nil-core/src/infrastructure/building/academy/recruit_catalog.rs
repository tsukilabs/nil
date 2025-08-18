// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::infrastructure::Infrastructure;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::military::unit::{AcademyUnitId, Unit, UnitBox};
use crate::resources::{Maintenance, Resources, Workforce};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademyRecruitCatalog {
  pikeman: AcademyRecruitCatalogEntry,
  swordsman: AcademyRecruitCatalogEntry,
  axeman: AcademyRecruitCatalogEntry,
  archer: AcademyRecruitCatalogEntry,
}

impl AcademyRecruitCatalog {
  pub fn new(infra: &Infrastructure) -> Self {
    macro_rules! make_entry {
      ($id:ident) => {{
        let unit = UnitBox::from(AcademyUnitId::$id);
        AcademyRecruitCatalogEntry::new(unit.as_dyn(), infra)
      }};
    }

    Self {
      pikeman: make_entry!(Pikeman),
      swordsman: make_entry!(Swordsman),
      axeman: make_entry!(Axeman),
      archer: make_entry!(Archer),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum AcademyRecruitCatalogEntry {
  /// Unit is available for recruitment.
  Available {
    recipe: Box<AcademyRecruitCatalogRecipe>,
  },
  /// City does not meet the requirements for recruitment.
  Unmet {
    requirements: InfrastructureRequirements,
  },
}

impl AcademyRecruitCatalogEntry {
  fn new(unit: &dyn Unit, infrastructure: &Infrastructure) -> Self {
    let infra_req = unit.infrastructure_requirements();
    if !infra_req.has_required_levels(infrastructure) {
      return Self::Unmet { requirements: infra_req.clone() };
    }

    let chunk = unit.chunk();
    let recipe = Box::new(AcademyRecruitCatalogRecipe {
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
pub struct AcademyRecruitCatalogRecipe {
  resources: Resources,
  maintenance: Maintenance,
  workforce: Workforce,
  requirements: InfrastructureRequirements,
}
