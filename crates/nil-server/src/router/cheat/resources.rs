// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::ruler::Ruler;
use nil_payload::cheat::resources::*;

pub async fn get_resources(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatGetResourcesRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world(req.world, |world| world.cheat_get_resources(&ruler))
    .await
    .try_map_left(|resources| res!(OK, Json(resources)))
    .into_inner()
}

pub async fn set_food(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetFoodRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| world.cheat_set_food(&ruler, req.food))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_iron(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetIronRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| world.cheat_set_iron(&ruler, req.iron))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_max_food(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetMaxFoodRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| world.cheat_set_max_food(&ruler))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_max_iron(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetMaxIronRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| world.cheat_set_max_iron(&ruler))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_max_resources(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetMaxResourcesRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| world.cheat_set_max_resources(&ruler))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_max_silo_resources(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetMaxSiloResourcesRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| {
      world.cheat_set_max_silo_resources(&ruler)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_max_stone(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetMaxStoneRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| world.cheat_set_max_stone(&ruler))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_max_warehouse_resources(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetMaxWarehouseResourcesRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| {
      world.cheat_set_max_warehouse_resources(&ruler)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_max_wood(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetMaxWoodRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| world.cheat_set_max_wood(&ruler))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_resources(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetResourcesRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| {
      world.cheat_set_resources(&ruler, req.resources)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_stone(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetStoneRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| world.cheat_set_stone(&ruler, req.stone))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_wood(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetWoodRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player));

  app
    .world_mut(req.world, |world| world.cheat_set_wood(&ruler, req.wood))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
