// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::{EitherExt, from_err};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use itertools::Itertools;
use nil_payload::report::*;
use nil_util::iter::IterExt;

pub async fn forward(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<ForwardReportRequest>,
) -> Response {
  if player == req.recipient {
    return res!(FORBIDDEN);
  }

  app
    .world_mut(req.world, |world| {
      world.forward_report(req.id, req.recipient)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn get(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetReportRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let world = world.read().await;
      let manager = world.report_manager();
      if manager
        .reports_of(&player.0)
        .any(|id| id == req.id)
      {
        manager
          .report(req.id)
          .map(|report| res!(OK, Json(report.clone())))
          .unwrap_or_else(from_err)
      } else {
        res!(FORBIDDEN)
      }
    }
    Err(err) => from_err(err),
  }
}

pub async fn get_by(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetReportsRequest>,
) -> Response {
  if req.ids.is_empty() {
    return res!(OK, Json(Vec::<()>::new()));
  }

  match app.get(req.world) {
    Ok(world) => {
      let world = world.read().await;
      let manager = world.report_manager();
      let player_reports = manager.reports_of(&player.0).collect_set();

      let reports = manager
        .reports_by(|(id, _)| req.ids.contains(&id))
        .filter(|report| player_reports.contains(&report.id()))
        .take(req.limit.unwrap_or(1_000))
        .sorted_unstable_by_key(|report| report.id())
        .rev()
        .cloned()
        .collect_vec();

      res!(OK, Json(reports))
    }
    Err(err) => from_err(err),
  }
}

pub async fn remove(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<RemoveReportRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      world.remove_report_of(req.id, &player.0);
    })
    .await
    .map_left(|()| res!(OK))
    .into_inner()
}
