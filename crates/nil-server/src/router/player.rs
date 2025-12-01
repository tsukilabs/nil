// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Extension, Json, Path, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use itertools::Itertools;
use nil_core::player::{Player, PlayerId, PublicPlayer};
use nil_payload::player::{GetPlayerRequest, SetPlayerStatusRequest, SpawnPlayerRequest};

pub async fn exists(State(app): State<App>, Path(id): Path<PlayerId>) -> Response {
  app
    .world(|world| world.has_player(&id))
    .map(|yes| res!(OK, Json(yes)))
    .await
}

pub async fn get(State(app): State<App>, Json(req): Json<GetPlayerRequest>) -> Response {
  app
    .player_manager(|pm| pm.player(&req.id).cloned())
    .map_ok(|player| res!(OK, Json(player)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_all(State(app): State<App>) -> Response {
  app
    .player_manager(|pm| pm.players().cloned().collect_vec())
    .map(|players| res!(OK, Json(players)))
    .await
}

pub async fn get_all_public(State(app): State<App>) -> Response {
  app
    .player_manager(|pm| {
      pm.players()
        .map(PublicPlayer::from)
        .collect_vec()
    })
    .map(|players| res!(OK, Json(players)))
    .await
}

pub async fn get_coords(State(app): State<App>, Path(id): Path<PlayerId>) -> Response {
  app
    .continent(|k| k.coords_of(id).collect_vec())
    .map(|coords| res!(OK, Json(coords)))
    .await
}

pub async fn get_maintenance(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world(|world| world.get_player_maintenance(&player))
    .map_ok(|maintenance| res!(OK, Json(maintenance)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_military(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world(|world| world.get_player_military(&player))
    .map_ok(|military| res!(OK, Json(military)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_public(State(app): State<App>, Path(id): Path<PlayerId>) -> Response {
  app
    .player_manager(|pm| pm.player(&id).map(PublicPlayer::from))
    .map_ok(|player| res!(OK, Json(player)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_status(State(app): State<App>, Path(id): Path<PlayerId>) -> Response {
  app
    .player_manager(|pm| pm.player(&id).map(Player::status))
    .map_ok(|status| res!(OK, Json(status)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_storage_capacity(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
) -> Response {
  app
    .world(|world| world.get_storage_capacity(player.0))
    .map_ok(|capacity| res!(OK, Json(capacity)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_status(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<SetPlayerStatusRequest>,
) -> Response {
  app
    .world_mut(|world| world.set_player_status(&player, req.status))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn spawn(State(app): State<App>, Json(req): Json<SpawnPlayerRequest>) -> Response {
  app
    .world_mut(|world| world.spawn_player(Player::new(req.options)))
    .map_ok(|()| res!(CREATED))
    .unwrap_or_else(from_core_err)
    .await
}
