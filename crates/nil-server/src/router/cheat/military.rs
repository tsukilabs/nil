// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::ruler::Ruler;
use nil_core::world::World;
use nil_payload::request::cheat::military::*;
use nil_payload::response::cheat::military::*;

pub async fn get_idle_armies_at(
  State(app): State<App>,
  Json(req): Json<CheatGetIdleArmiesAtRequest>,
) -> Response {
  app
    .world(req.world, |world| world.cheat_get_idle_armies_at(req.coord))
    .await
    .try_map_left(|armies| res!(OK, CheatGetIdleArmiesAtResponse(armies)))
    .into_inner()
}

pub async fn get_idle_personnel_at(
  State(app): State<App>,
  Json(req): Json<CheatGetIdlePersonnelAtRequest>,
) -> Response {
  app
    .world(req.world, |world| {
      world.cheat_get_idle_personnel_at(req.coord)
    })
    .await
    .try_map_left(|personnel| res!(OK, CheatGetIdlePersonnelAtResponse(personnel)))
    .into_inner()
}

pub async fn get_maneuvers(
  State(app): State<App>,
  Json(req): Json<CheatGetManeuversRequest>,
) -> Response {
  app
    .world(req.world, World::cheat_get_maneuvers)
    .await
    .try_map_left(|maneuvers| res!(OK, CheatGetManeuversResponse(maneuvers)))
    .into_inner()
}

pub async fn get_maneuvers_of(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatGetManeuversOfRequest>,
) -> Response {
  let ruler = req
    .ruler
    .unwrap_or_else(|| Ruler::from(player.0));

  app
    .world(req.world, |world| world.cheat_get_maneuvers_of(ruler))
    .await
    .try_map_left(|maneuvers| res!(OK, CheatGetManeuversOfResponse(maneuvers)))
    .into_inner()
}

pub async fn spawn_personnel(
  State(app): State<App>,
  Json(req): Json<CheatSpawnPersonnelRequest>,
) -> Response {
  app
    .world_blocking_mut(req.world, move |world| {
      world.cheat_spawn_personnel(req.coord, req.personnel, req.ruler)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
