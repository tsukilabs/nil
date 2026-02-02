// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::cheat::round::*;
use std::num::NonZeroU8;

pub async fn skip(State(app): State<App>, Json(mut req): Json<CheatSkipRoundRequest>) -> Response {
  if app.server_kind().is_remote() && !cfg!(debug_assertions) {
    req.amount = NonZeroU8::MIN;
  }

  app
    .world_blocking_mut(req.world, move |world| world.cheat_skip_round(req.amount))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
