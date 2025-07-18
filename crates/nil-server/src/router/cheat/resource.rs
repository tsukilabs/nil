// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::resource::prelude::*;

pub async fn set_food(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(food): Json<Food>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_food(player.0, food))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_iron(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(iron): Json<Iron>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_iron(player.0, iron))
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
  Json(resources): Json<Resources>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_resources(player.0, resources))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_stone(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(stone): Json<Stone>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_stone(player.0, stone))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_wood(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(wood): Json<Wood>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_wood(player.0, wood))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}
