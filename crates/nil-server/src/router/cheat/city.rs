// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::cheat::city::*;

pub async fn set_stability(
  State(app): State<App>,
  Json(req): Json<CheatSetStabilityRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      world.cheat_set_stability(req.coord, req.stability)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
