// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::ruler::Ruler;
use nil_payload::cheat::infrastructure::{
  CheatGetAcademyRecruitQueueRequest,
  CheatGetInfrastructureRequest,
  CheatGetPrefectureBuildQueueRequest,
  CheatGetStableRecruitQueueRequest,
  CheatGetStorageCapacityRequest,
  CheatSetBuildingLevelRequest,
  CheatSetMaxInfrastructureRequest,
};

pub async fn get_academy_recruit_queue(
  State(app): State<App>,
  Json(req): Json<CheatGetAcademyRecruitQueueRequest>,
) -> Response {
  app
    .world(|world| world.cheat_get_academy_recruit_queue(req.coord))
    .map_ok(|queue| res!(OK, Json(queue)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_infrastructure(
  State(app): State<App>,
  Json(req): Json<CheatGetInfrastructureRequest>,
) -> Response {
  app
    .world(|world| world.cheat_get_infrastructure(req.coord))
    .map_ok(|infrastructure| res!(OK, Json(infrastructure)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_prefecture_build_queue(
  State(app): State<App>,
  Json(req): Json<CheatGetPrefectureBuildQueueRequest>,
) -> Response {
  app
    .world(|world| world.cheat_get_prefecture_build_queue(req.coord))
    .map_ok(|queue| res!(OK, Json(queue)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_stable_recruit_queue(
  State(app): State<App>,
  Json(req): Json<CheatGetStableRecruitQueueRequest>,
) -> Response {
  app
    .world(|world| world.cheat_get_stable_recruit_queue(req.coord))
    .map_ok(|queue| res!(OK, Json(queue)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_storage_capacity(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatGetStorageCapacityRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world(|world| world.cheat_get_storage_capacity(ruler))
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
