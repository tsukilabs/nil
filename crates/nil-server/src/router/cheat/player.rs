// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::world::cheat;
use nil_payload::request::cheat::player::*;
use nil_payload::response::cheat::player::*;

pub async fn get_player(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatGetPlayerRequest>,
) -> Response {
  let player = req.player.unwrap_or(player.0);
  app
    .world(req.world, |world| cheat::get_player(world, &player))
    .await
    .try_map_left(|player| res!(OK, CheatGetPlayerResponse(player)))
    .into_inner()
}

pub async fn get_players(
  State(app): State<App>,
  Json(req): Json<CheatGetPlayersRequest>,
) -> Response {
  app
    .world(req.world, cheat::get_players)
    .await
    .try_map_left(|players| res!(OK, CheatGetPlayersResponse(players)))
    .into_inner()
}
