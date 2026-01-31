// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::cheat::round::*;

pub async fn skip(State(app): State<App>, Json(req): Json<CheatSkipRoundRequest>) -> Response {
  app
    .world_blocking_mut(req.world, move |world| world.cheat_skip_round(req.amount))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
