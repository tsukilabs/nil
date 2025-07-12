// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::infrastructure::building::{BuildingId, BuildingLevel};
use nil_core::village::Coord;

pub async fn set_building_level(
  State(app): State<App>,
  Json((coord, id, level)): Json<(Coord, BuildingId, BuildingLevel)>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_building_level(coord, id, level))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_max_infrastructure(State(app): State<App>, Json(coord): Json<Coord>) -> Response {
  app
    .world_mut(|world| world.cheat_set_max_infrastructure(coord))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}
