// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::decl_recruit_catalog_entry;
use crate::infrastructure::Infrastructure;
use crate::infrastructure::requirements::InfrastructureRequirements;
use crate::military::unit::{StableUnitId, Unit, UnitBox};
use crate::resources::Resources;
use crate::resources::maintenance::Maintenance;
use crate::resources::workforce::Workforce;
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

decl_recruit_catalog_entry!(Stable);
