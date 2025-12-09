// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::BattleResult;
use crate::city::PublicCity;
use crate::report::ReportId;
use crate::resources::Resources;
use crate::ruler::Ruler;
use jiff::Zoned;
use nil_core_macros::Report;
use serde::{Deserialize, Serialize};

#[derive(Report, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleReport {
  id: ReportId,
  attacker: Ruler,
  defender: Ruler,
  result: BattleResult,
  city: PublicCity,
  hauled_resources: Resources,
  timestamp: Zoned,
}

#[bon::bon]
impl BattleReport {
  #[builder]
  pub fn new(
    attacker: Ruler,
    defender: Ruler,
    result: BattleResult,
    city: PublicCity,
    hauled_resources: Resources,
  ) -> Self {
    Self {
      id: ReportId::new(),
      attacker,
      defender,
      result,
      city,
      hauled_resources,
      timestamp: Zoned::now(),
    }
  }

  #[inline]
  pub fn attacker(&self) -> &Ruler {
    &self.attacker
  }

  #[inline]
  pub fn defender(&self) -> &Ruler {
    &self.defender
  }

  #[inline]
  pub fn result(&self) -> &BattleResult {
    &self.result
  }

  #[inline]
  pub fn city(&self) -> &PublicCity {
    &self.city
  }

  #[inline]
  pub fn hauled_resources(&self) -> &Resources {
    &self.hauled_resources
  }
}
