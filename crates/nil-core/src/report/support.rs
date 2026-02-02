// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::military::army::ArmyPersonnel;
use crate::report::ReportId;
use crate::round::RoundId;
use crate::ruler::Ruler;
use bon::Builder;
use jiff::Zoned;
use nil_core_macros::Report;
use serde::{Deserialize, Serialize};

#[derive(Report, Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportReport {
  #[builder(skip = ReportId::new())]
  id: ReportId,
  sender: Ruler,
  receiver: Ruler,
  origin: Coord,
  destination: Coord,
  personnel: ArmyPersonnel,
  round: RoundId,
  #[builder(skip = Zoned::now())]
  time: Zoned,
}

impl SupportReport {
  #[inline]
  pub fn sender(&self) -> &Ruler {
    &self.sender
  }

  #[inline]
  pub fn receiver(&self) -> &Ruler {
    &self.receiver
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
  pub fn personnel(&self) -> &ArmyPersonnel {
    &self.personnel
  }
}
