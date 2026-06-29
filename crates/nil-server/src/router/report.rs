// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_payload::request::report::*;

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
      world.forward_report(req.recipient, req.report)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
