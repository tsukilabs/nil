// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_payload::cheat::resources::{
  CheatGetResourcesRequest,
  CheatSetFoodRequest,
  CheatSetIronRequest,
  CheatSetResourcesRequest,
  CheatSetStoneRequest,
  CheatSetWoodRequest,
};

pub async fn get_resources(
  State(app): State<App>,
  Json(req): Json<CheatGetResourcesRequest>,
) -> Response {
  app
    .world(|world| world.cheat_get_resources(req.ruler))
    .map_ok(|resources| res!(OK, Json(resources)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_food(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetFoodRequest>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_food(player.0, req.food))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_iron(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetIronRequest>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_iron(player.0, req.iron))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_max_food(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_max_food(player.0))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_max_iron(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_max_iron(player.0))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_max_resources(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_max_resources(player.0))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_max_silo_resources(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_max_silo_resources(player.0))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_max_stone(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_max_stone(player.0))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_max_warehouse_resources(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_max_warehouse_resources(player.0))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_max_wood(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_max_wood(player.0))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_resources(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetResourcesRequest>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_resources(player.0, req.resources))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_stone(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetStoneRequest>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_stone(player.0, req.stone))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_wood(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetWoodRequest>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_wood(player.0, req.wood))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}
