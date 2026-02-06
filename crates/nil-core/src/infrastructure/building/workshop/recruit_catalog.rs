// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::decl_recruit_catalog_entry;
use crate::infrastructure::Infrastructure;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::military::unit::{Unit, UnitBox, WorkshopUnitId};
use crate::resources::Resources;
use crate::resources::maintenance::Maintenance;
use crate::resources::workforce::Workforce;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkshopRecruitCatalog {
  ram: WorkshopRecruitCatalogEntry,
}

impl WorkshopRecruitCatalog {
  pub fn new(infra: &Infrastructure) -> Self {
    macro_rules! make_entry {
      ($id:ident) => {{
        let unit = UnitBox::from(WorkshopUnitId::$id);
        WorkshopRecruitCatalogEntry::new(unit.as_dyn(), infra)
      }};
    }

    Self { ram: make_entry!(Ram) }
  }
}

decl_recruit_catalog_entry!(Workshop);
