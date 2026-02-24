// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::world::World;
use nil_payload::cheat::player::*;

pub async fn get_player(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatGetPlayerRequest>,
) -> Response {
  let player = req.player.unwrap_or(player.0);
  app
    .world(req.world, |world| world.cheat_get_player(&player))
    .await
    .try_map_left(|player| res!(OK, Json(player)))
    .into_inner()
}

pub async fn get_players(
  State(app): State<App>,
  Json(req): Json<CheatGetPlayersRequest>,
) -> Response {
  app
    .world(req.world, World::cheat_get_players)
    .await
    .try_map_left(|players| res!(OK, Json(players)))
    .into_inner()
}
