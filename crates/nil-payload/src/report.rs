// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use nil_core::report::ReportId;
use nil_core::world::WorldId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReportRequest {
  pub world: WorldId,
  pub id: ReportId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReportsRequest {
  pub world: WorldId,
  pub ids: HashSet<ReportId>,
  #[serde(default)]
  pub limit: Option<usize>,
}
