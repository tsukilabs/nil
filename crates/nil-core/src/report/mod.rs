// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod battle;
pub mod support;

use crate::round::RoundId;
use battle::BattleReport;
use jiff::Zoned;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use support::SupportReport;
use uuid::Uuid;

pub trait Report {
  fn id(&self) -> ReportId;
  fn round(&self) -> RoundId;
  fn created_at(&self) -> &Zoned;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[remain::sorted]
pub enum ReportKind {
  Battle { report: Arc<BattleReport> },
  Support { report: Arc<SupportReport> },
}

impl ReportKind {
  pub fn as_dyn(&self) -> &dyn Report {
    match self {
      Self::Battle { report } => report.as_ref(),
      Self::Support { report } => report.as_ref(),
    }
  }

  #[inline]
  pub fn id(&self) -> ReportId {
    self.as_dyn().id()
  }
}

#[must_use]
#[derive(
  Clone,
  Copy,
  Debug,
  derive_more::Display,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Deserialize,
  Serialize,
)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ReportId(Uuid);

impl ReportId {
  #[inline]
  pub fn new() -> Self {
    Self(Uuid::now_v7())
  }
}

impl Default for ReportId {
  fn default() -> Self {
    Self::new()
  }
}
