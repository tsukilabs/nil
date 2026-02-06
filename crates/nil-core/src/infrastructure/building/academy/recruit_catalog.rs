// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::decl_recruit_catalog_entry;
use crate::infrastructure::Infrastructure;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::military::unit::{AcademyUnitId, Unit, UnitBox};
use crate::resources::Resources;
use crate::resources::maintenance::Maintenance;
use crate::resources::workforce::Workforce;
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

decl_recruit_catalog_entry!(Academy);
