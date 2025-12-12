// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_payload::cheat::npc::{CheatGetEthicsRequest, CheatSpawnBotRequest};

pub async fn get_ethics(
  State(app): State<App>,
  Json(req): Json<CheatGetEthicsRequest>,
) -> Response {
  app
    .world_mut(|world| world.cheat_get_ethics(&req.ruler))
    .map_ok(|ethics| res!(OK, Json(ethics)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn spawn_bot(State(app): State<App>, Json(req): Json<CheatSpawnBotRequest>) -> Response {
  app
    .world_mut(|world| world.cheat_spawn_bot(&req.name))
    .map_ok(|id| res!(OK, Json(id)))
    .unwrap_or_else(from_core_err)
    .await
}
