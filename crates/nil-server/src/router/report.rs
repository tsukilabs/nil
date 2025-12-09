// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use itertools::Itertools;
use nil_payload::report::{GetReportRequest, GetReportsRequest};

pub async fn get(State(app): State<App>, Json(req): Json<GetReportRequest>) -> Response {
  app
    .report_manager(|rm| rm.report(req.id).cloned())
    .map_ok(|report| res!(OK, Json(report)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_by(State(app): State<App>, Json(req): Json<GetReportsRequest>) -> Response {
  let world = app.world.read().await;
  let reports = world
    .report()
    .reports_by(|(id, _)| req.ids.contains(&id))
    .take(req.limit.unwrap_or(1_000))
    .cloned()
    .collect_vec();

  res!(OK, Json(reports))
}
