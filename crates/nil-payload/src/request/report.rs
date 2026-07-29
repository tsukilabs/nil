// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::player::PlayerId;
use nil_core::report::ReportKind;
use nil_core::world::config::WorldId;
use serde::{Deserialize, Serialize};

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct ForwardReportRequest {
  #[builder(start_fn, into)]
  pub world: WorldId,
  #[builder(into)]
  pub recipient: PlayerId,
  #[builder(into)]
  pub report: ReportKind,
}
