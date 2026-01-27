// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use crate::server::local;
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

pub async fn save_local(
  State(app): State<App>,
  Json(req): Json<SaveLocalWorldRequest>,
) -> Response {
  if app.server_kind().is_local() {
    let f = move |bytes: Vec<u8>| {
      if let Err(_err) = local::save_local(req.path, &bytes) {
        #[cfg(debug_assertions)]
        tracing::error!(message = %_err, error = ?_err);
      }
    };

    app
      .world_mut(req.world, move |world| world.save(f))
      .await
      .map_left(|()| res!(OK))
      .into_inner()
  } else {
    res!(FORBIDDEN)
  }
}
