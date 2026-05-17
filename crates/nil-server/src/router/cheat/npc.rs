// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::response::EitherExt;
use crate::{bail_if_max_chars_exceeded, res};
use axum::extract::{Json, State};
use axum::response::Response;
use nil_core::ethic::Ethics;
use nil_payload::request::cheat::npc::*;
use nil_payload::response::cheat::npc::*;

pub async fn get_ethics(
  State(app): State<App>,
  Json(req): Json<CheatGetEthicsRequest>,
) -> Response {
  app
    .world(req.world, |world| world.cheat_get_ethics(&req.ruler))
    .await
    .try_map_left(|ethics| res!(OK, CheatGetEthicsResponse(ethics)))
    .into_inner()
}

pub async fn set_bot_ethics(
  State(app): State<App>,
  Json(req): Json<CheatSetBotEthicsRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      let ethics = req.ethics.unwrap_or_else(Ethics::random);
      world.cheat_set_bot_ethics(&req.id, ethics)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn spawn_bot(State(app): State<App>, Json(req): Json<CheatSpawnBotRequest>) -> Response {
  bail_if_max_chars_exceeded!(req.name, 50);
  app
    .world_blocking_mut(req.world, move |world| {
      let infrastructure = req.infrastructure.unwrap_or_default();
      world.cheat_spawn_bot(&req.name, infrastructure)
    })
    .await
    .try_map_left(|id| res!(OK, CheatSpawnBotResponse(id)))
    .into_inner()
}
