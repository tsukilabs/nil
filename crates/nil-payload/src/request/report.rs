// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::player::PlayerId;
use nil_core::report::ReportId;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ts_rs::TS;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ForwardReportRequest {
  pub world: WorldId,
  pub id: ReportId,
  pub recipient: PlayerId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GetReportRequest {
  pub world: WorldId,
  pub id: ReportId,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, optional_fields = nullable)]
pub struct GetReportsRequest {
  pub world: WorldId,
  pub ids: HashSet<ReportId>,
  #[serde(default)]
  pub limit: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RemoveReportRequest {
  pub world: WorldId,
  pub id: ReportId,
}
