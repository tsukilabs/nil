// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_payload::cheat::round::CheatSkipRoundRequest;

pub async fn skip(State(app): State<App>, Json(req): Json<CheatSkipRoundRequest>) -> Response {
  app
    .world_mut(|world| world.cheat_skip_round(req.amount.get()))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}
