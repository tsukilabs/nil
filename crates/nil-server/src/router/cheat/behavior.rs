// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::cheat::behavior::*;

pub async fn get_build_steps(
  State(app): State<App>,
  Json(req): Json<CheatGetBuildStepsRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| world.cheat_get_build_steps(req.coord))
    .await
    .try_map_left(|steps| res!(OK, Json(steps)))
    .into_inner()
}
