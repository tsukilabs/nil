// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::ruler::Ruler;
use nil_payload::cheat::infrastructure::*;

pub async fn get_academy_recruit_queue(
  State(app): State<App>,
  Json(req): Json<CheatGetAcademyRecruitQueueRequest>,
) -> Response {
  app
    .world(req.world, |world| {
      world.cheat_get_academy_recruit_queue(req.coord)
    })
    .await
    .try_map_left(|queue| res!(OK, Json(queue)))
    .into_inner()
}

pub async fn get_infrastructure(
  State(app): State<App>,
  Json(req): Json<CheatGetInfrastructureRequest>,
) -> Response {
  app
    .world(req.world, |world| world.cheat_get_infrastructure(req.coord))
    .await
    .try_map_left(|infrastructure| res!(OK, Json(infrastructure)))
    .into_inner()
}

pub async fn get_prefecture_build_queue(
  State(app): State<App>,
  Json(req): Json<CheatGetPrefectureBuildQueueRequest>,
) -> Response {
  app
    .world(req.world, |world| {
      world.cheat_get_prefecture_build_queue(req.coord)
    })
    .await
    .try_map_left(|queue| res!(OK, Json(queue)))
    .into_inner()
}

pub async fn get_stable_recruit_queue(
  State(app): State<App>,
  Json(req): Json<CheatGetStableRecruitQueueRequest>,
) -> Response {
  app
    .world(req.world, |world| {
      world.cheat_get_stable_recruit_queue(req.coord)
    })
    .await
    .try_map_left(|queue| res!(OK, Json(queue)))
    .into_inner()
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
    .world(req.world, |world| world.cheat_get_storage_capacity(ruler))
    .await
    .try_map_left(|capacity| res!(OK, Json(capacity)))
    .into_inner()
}

pub async fn set_building_level(
  State(app): State<App>,
  Json(req): Json<CheatSetBuildingLevelRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      world.cheat_set_building_level(req.coord, req.id, req.level)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_max_infrastructure(
  State(app): State<App>,
  Json(req): Json<CheatSetMaxInfrastructureRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      world.cheat_set_max_infrastructure(req.coord)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
