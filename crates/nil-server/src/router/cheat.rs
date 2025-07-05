// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::resource::{Food, Iron, Resources, Stone, Wood};
use nil_core::village::{Coord, Stability};

pub async fn set_food(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(food): Json<Food>,
) -> Response {
  let player = current_player.0;
  app
    .world_mut(|world| world.cheat_set_food(player, food))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_iron(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(iron): Json<Iron>,
) -> Response {
  let player = current_player.0;
  app
    .world_mut(|world| world.cheat_set_iron(player, iron))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_max_resources(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
) -> Response {
  let player = current_player.0;
  app
    .world_mut(|world| world.cheat_set_max_resources(player))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_resources(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(resources): Json<Resources>,
) -> Response {
  let player = current_player.0;
  app
    .world_mut(|world| world.cheat_set_resources(player, resources))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_stability(
  State(app): State<App>,
  Json((coord, stability)): Json<(Coord, Stability)>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_stability(coord, stability))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_stone(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(stone): Json<Stone>,
) -> Response {
  let player = current_player.0;
  app
    .world_mut(|world| world.cheat_set_stone(player, stone))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_wood(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(wood): Json<Wood>,
) -> Response {
  let player = current_player.0;
  app
    .world_mut(|world| world.cheat_set_wood(player, wood))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}
