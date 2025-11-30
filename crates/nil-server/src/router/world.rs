// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::FutureExt;
use nil_core::world::World;
use nil_payload::world::SaveWorldRequest;

pub async fn get_config(State(app): State<App>) -> Response {
  app
    .world(World::config)
    .map(|world| res!(OK, Json(world)))
    .await
}

pub async fn get_stats(State(app): State<App>) -> Response {
  app
    .world(World::stats)
    .map(|stats| res!(OK, Json(stats)))
    .await
}

pub async fn save(State(app): State<App>, Json(req): Json<SaveWorldRequest>) -> Response {
  app
    .world_mut(|world| world.save(req.path))
    .map(|()| res!(OK))
    .await
}
