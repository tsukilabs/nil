// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_payload::cheat::infrastructure::{
  CheatGetStorageCapacityRequest,
  CheatSetBuildingLevelRequest,
  CheatSetMaxInfrastructureRequest,
};

pub async fn get_storage_capacity(
  State(app): State<App>,
  Json(req): Json<CheatGetStorageCapacityRequest>,
) -> Response {
  app
    .world(|world| world.cheat_get_storage_capacity(req.ruler))
    .map_ok(|capacity| res!(OK, Json(capacity)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_building_level(
  State(app): State<App>,
  Json(req): Json<CheatSetBuildingLevelRequest>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_building_level(req.coord, req.id, req.level))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_max_infrastructure(
  State(app): State<App>,
  Json(req): Json<CheatSetMaxInfrastructureRequest>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_max_infrastructure(req.coord))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}
