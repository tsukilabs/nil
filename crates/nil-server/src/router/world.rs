// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_core::world::World;
use nil_payload::world::*;

pub async fn get_config(
  State(app): State<App>,
  Json(req): Json<GetWorldConfigRequest>,
) -> Response {
  app
    .world(req.world, |world| world.config().clone())
    .await
    .map_left(|world| res!(OK, Json(world)))
    .into_inner()
}

pub async fn get_stats(State(app): State<App>, Json(req): Json<GetWorldStatsRequest>) -> Response {
  app
    .world(req.world, World::stats)
    .await
    .map_left(|stats| res!(OK, Json(stats)))
    .into_inner()
}

pub async fn save(State(app): State<App>, Json(req): Json<SaveWorldRequest>) -> Response {
  app
    .world_mut(req.world, |world| world.save(req.path))
    .await
    .map_left(|()| res!(OK))
    .into_inner()
}
