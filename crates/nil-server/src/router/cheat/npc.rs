// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::cheat::npc::*;

pub async fn get_ethics(
  State(app): State<App>,
  Json(req): Json<CheatGetEthicsRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| world.cheat_get_ethics(&req.ruler))
    .await
    .try_map_left(|ethics| res!(OK, Json(ethics)))
    .into_inner()
}

pub async fn set_bot_ethics(
  State(app): State<App>,
  Json(req): Json<CheatSetBotEthicsRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      world.cheat_set_bot_ethics(&req.id, req.ethics)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn spawn_bot(State(app): State<App>, Json(req): Json<CheatSpawnBotRequest>) -> Response {
  app
    .world_mut(req.world, |world| world.cheat_spawn_bot(&req.name))
    .await
    .try_map_left(|id| res!(OK, Json(id)))
    .into_inner()
}
