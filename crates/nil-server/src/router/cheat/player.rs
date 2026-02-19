// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::cheat::player::*;

pub async fn get_players(
  State(app): State<App>,
  Json(req): Json<CheatGetPlayersRequest>,
) -> Response {
  app
    .world(req.world, |world| world.cheat_get_players())
    .await
    .try_map_left(|players| res!(OK, Json(players)))
    .into_inner()
}
