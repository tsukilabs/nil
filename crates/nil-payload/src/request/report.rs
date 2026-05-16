// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::player::PlayerId;
use nil_core::report::ReportId;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct ForwardReportRequest {
  pub world: WorldId,
  pub id: ReportId,
  #[builder(into)]
  pub recipient: PlayerId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct GetReportRequest {
  pub world: WorldId,
  pub id: ReportId,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct GetReportsRequest {
  pub world: WorldId,
  #[serde(default)]
  #[builder(default, with = FromIterator::from_iter)]
  pub ids: HashSet<ReportId>,
  #[serde(default)]
  pub limit: Option<u16>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct RemoveReportRequest {
  pub world: WorldId,
  pub id: ReportId,
}
