// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::cheat::military::*;

pub async fn get_idle_armies_at(
  State(app): State<App>,
  Json(req): Json<CheatGetIdleArmiesAtRequest>,
) -> Response {
  app
    .world(req.world, |world| world.cheat_get_idle_armies_at(req.coord))
    .await
    .try_map_left(|armies| res!(OK, Json(armies)))
    .into_inner()
}

pub async fn get_idle_personnel_at(
  State(app): State<App>,
  Json(req): Json<CheatGetIdlePersonnelAtRequest>,
) -> Response {
  app
    .world(req.world, |world| {
      world.cheat_get_idle_personnel_at(req.coord)
    })
    .await
    .try_map_left(|personnel| res!(OK, Json(personnel)))
    .into_inner()
}

pub async fn spawn_personnel(
  State(app): State<App>,
  Json(req): Json<CheatSpawnPersonnelRequest>,
) -> Response {
  app
    .world_blocking_mut(req.world, move |world| {
      world.cheat_spawn_personnel(req.coord, req.personnel, req.ruler)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
