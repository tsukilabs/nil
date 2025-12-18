// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::cheat::military::*;

pub async fn spawn_personnel(
  State(app): State<App>,
  Json(req): Json<CheatSpawnPersonnelRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      world.cheat_spawn_personnel(req.coord, req.personnel, req.ruler)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
