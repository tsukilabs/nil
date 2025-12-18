// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use itertools::Itertools;
use nil_core::player::{Player, PublicPlayer};
use nil_payload::player::*;

pub async fn exists(State(app): State<App>, Json(req): Json<PlayerExistsRequest>) -> Response {
  app
    .world(req.world, |world| world.has_player(&req.id))
    .await
    .map_left(|yes| res!(OK, Json(yes)))
    .into_inner()
}

pub async fn get(State(app): State<App>, Json(req): Json<GetPlayerRequest>) -> Response {
  app
    .player_manager(req.world, |pm| pm.player(&req.id).cloned())
    .await
    .try_map_left(|player| res!(OK, Json(player)))
    .into_inner()
}

pub async fn get_all(State(app): State<App>, Json(req): Json<GetPlayersRequest>) -> Response {
  app
    .player_manager(req.world, |pm| pm.players().cloned().collect_vec())
    .await
    .map_left(|players| res!(OK, Json(players)))
    .into_inner()
}

pub async fn get_all_public(
  State(app): State<App>,
  Json(req): Json<GetPublicPlayersRequest>,
) -> Response {
  app
    .player_manager(req.world, |pm| {
      pm.players()
        .map(PublicPlayer::from)
        .collect_vec()
    })
    .await
    .map_left(|players| res!(OK, Json(players)))
    .into_inner()
}

pub async fn get_coords(
  State(app): State<App>,
  Json(req): Json<GetPlayerCoordsRequest>,
) -> Response {
  app
    .continent(req.world, |k| k.coords_of(req.id).collect_vec())
    .await
    .map_left(|coords| res!(OK, Json(coords)))
    .into_inner()
}

pub async fn get_maintenance(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetPlayerMaintenanceRequest>,
) -> Response {
  app
    .world(req.world, |world| world.get_player_maintenance(&player))
    .await
    .try_map_left(|maintenance| res!(OK, Json(maintenance)))
    .into_inner()
}

pub async fn get_military(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetPlayerMilitaryRequest>,
) -> Response {
  app
    .world(req.world, |world| world.get_player_military(&player))
    .await
    .try_map_left(|military| res!(OK, Json(military)))
    .into_inner()
}

pub async fn get_public(
  State(app): State<App>,
  Json(req): Json<GetPublicPlayerRequest>,
) -> Response {
  app
    .player_manager(req.world, |pm| pm.player(&req.id).map(PublicPlayer::from))
    .await
    .try_map_left(|player| res!(OK, Json(player)))
    .into_inner()
}

pub async fn get_reports(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetPlayerReportsRequest>,
) -> Response {
  app
    .world(req.world, |world| world.get_player_reports(&player))
    .await
    .map_left(|reports| res!(OK, Json(reports)))
    .into_inner()
}

pub async fn get_status(
  State(app): State<App>,
  Json(req): Json<GetPlayerStatusRequest>,
) -> Response {
  app
    .player_manager(req.world, |pm| pm.player(&req.id).map(Player::status))
    .await
    .try_map_left(|status| res!(OK, Json(status)))
    .into_inner()
}

pub async fn get_storage_capacity(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetPlayerStorageCapacityRequest>,
) -> Response {
  app
    .world(req.world, |world| world.get_storage_capacity(player.0))
    .await
    .try_map_left(|capacity| res!(OK, Json(capacity)))
    .into_inner()
}

pub async fn set_status(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<SetPlayerStatusRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      world.set_player_status(&player, req.status)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn spawn(State(app): State<App>, Json(req): Json<SpawnPlayerRequest>) -> Response {
  app
    .world_mut(req.world, |world| {
      world.spawn_player(Player::new(req.options))
    })
    .await
    .try_map_left(|()| res!(CREATED))
    .into_inner()
}
