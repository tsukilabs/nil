// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_payload::request::cheat::city::*;

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

pub async fn spawn_city(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSpawnCityRequest>,
) -> Response {
  app
    .world_mut(req.world, move |world| {
      let ruler = req.ruler.unwrap_or_else(|| player.into());
      world.cheat_spawn_city(ruler, req.coord)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
