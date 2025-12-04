// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_payload::military::RequestManeuverRequest;

pub async fn request_maneuver(
  State(app): State<App>,
  Json(req): Json<RequestManeuverRequest>,
) -> Response {
  app
    .world_mut(|world| world.request_maneuver(req.request))
    .map_ok(|id| res!(OK, Json(id)))
    .unwrap_or_else(from_core_err)
    .await
}
