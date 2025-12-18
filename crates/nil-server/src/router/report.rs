// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use itertools::Itertools;
use nil_payload::report::*;

pub async fn get(State(app): State<App>, Json(req): Json<GetReportRequest>) -> Response {
  app
    .report_manager(req.world, |rm| rm.report(req.id).cloned())
    .await
    .try_map_left(|report| res!(OK, Json(report)))
    .into_inner()
}

pub async fn get_by(State(app): State<App>, Json(req): Json<GetReportsRequest>) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let world = world.read().await;
      let reports = world
        .report()
        .reports_by(|(id, _)| req.ids.contains(&id))
        .take(req.limit.unwrap_or(1_000))
        .sorted_unstable_by_key(|report| report.as_dyn().id())
        .rev()
        .cloned()
        .collect_vec();

      res!(OK, Json(reports))
    }
    Err(err) => Response::from(err),
  }
}
