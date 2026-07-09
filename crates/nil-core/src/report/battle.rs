// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::BattleResult;
use crate::continent::coord::Coord;
use crate::report::ReportId;
use crate::resources::Resources;
use crate::round::RoundId;
use crate::ruler::Ruler;
use bon::Builder;
use jiff::Zoned;
use nil_core_macros::Report;
use serde::{Deserialize, Serialize};

#[derive(Report, Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct BattleReport {
  #[builder(skip = ReportId::new())]
  id: ReportId,
  attacker: Ruler,
  defender: Ruler,
  origin: Coord,
  destination: Coord,
  result: BattleResult,
  hauled_resources: Resources,
  round: RoundId,
  #[builder(skip = Zoned::now())]
  time: Zoned,
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
  pub fn result(&self) -> &BattleResult {
    &self.result
  }

  #[inline]
  pub fn origin(&self) -> Coord {
    self.origin
  }

  #[inline]
  pub fn destination(&self) -> Coord {
    self.destination
  }

  #[inline]
  pub fn hauled_resources(&self) -> &Resources {
    &self.hauled_resources
  }
}
