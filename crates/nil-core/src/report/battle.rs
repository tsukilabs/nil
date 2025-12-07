// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::BattleResult;
use crate::report::Report;
use crate::resources::Resources;
use crate::ruler::Ruler;
use bon::Builder;
use jiff::Zoned;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BattleReport {
  attacker: Ruler,
  defenders: Box<[Ruler]>,
  hauled_resources: Resources,
  result: BattleResult,

  #[builder(skip = Zoned::now())]
  timestamp: Zoned,
}

impl BattleReport {
  #[inline]
  pub fn attacker(&self) -> &Ruler {
    &self.attacker
  }

  #[inline]
  pub fn defenders(&self) -> &[Ruler] {
    &self.defenders
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

impl Report for BattleReport {
  fn timestamp(&self) -> &Zoned {
    &self.timestamp
  }
}
