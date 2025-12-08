// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::BattleResult;
use crate::report::ReportId;
use crate::resources::Resources;
use crate::ruler::Ruler;
use bon::Builder;
use jiff::Zoned;
use nil_core_macros::Report;
use serde::{Deserialize, Serialize};

#[derive(Report, Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleReport {
  #[builder(skip)]
  id: ReportId,
  #[builder(skip = Zoned::now())]
  timestamp: Zoned,

  attacker: Ruler,
  defender: Ruler,
  hauled_resources: Resources,
  result: BattleResult,
}

impl BattleReport {
  #[inline]
  pub fn attacker(&self) -> &Ruler {
    &self.attacker
  }

  #[inline]
  pub fn defender(&self) -> &Ruler {
    &self.defender
  }

  #[inline]
  pub fn hauled_resources(&self) -> &Resources {
    &self.hauled_resources
  }

  #[inline]
  pub fn result(&self) -> &BattleResult {
    &self.result
  }
}
