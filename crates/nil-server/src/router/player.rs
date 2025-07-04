// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use itertools::Itertools;
use nil_core::player::{Player, PlayerId, PlayerOptions, PlayerStatus};

pub async fn exists(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .world(|world| world.has_player(&id))
    .map(|yes| res!(OK, Json(yes)))
    .await
}

pub async fn get(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .player_manager(|pm| pm.player(&id).cloned())
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

pub async fn get_coords(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .continent(|k| k.player_coords(&id).collect_vec())
    .map(|coords| res!(OK, Json(coords)))
    .await
}

pub async fn remove_guest(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .world_mut(|world| world.remove_guest(&id))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn set_status(
  State(app): State<App>,
  Json((id, status)): Json<(PlayerId, PlayerStatus)>,
) -> Response {
  app
    .world_mut(|world| world.set_player_status(&id, status))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn spawn(State(app): State<App>, Json(options): Json<PlayerOptions>) -> Response {
  app
    .world_mut(|world| world.spawn_player(Player::new(options)))
    .map_ok(|()| res!(CREATED))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn spawn_village(State(app): State<App>, Json(id): Json<PlayerId>) -> Response {
  app
    .world_mut(|world| world.spawn_player_village(id))
    .map_ok(|()| res!(CREATED))
    .unwrap_or_else(from_core_err)
    .await
}
