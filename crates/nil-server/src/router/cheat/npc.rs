// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, Path, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::npc::bot::BotId;
use nil_core::npc::precursor::PrecursorId;
use nil_payload::cheat::npc::CheatSpawnBotRequest;

pub async fn get_bot_resources(State(app): State<App>, Path(id): Path<BotId>) -> Response {
  app
    .world(|world| world.cheat_get_bot_resources(&id))
    .map_ok(|resources| res!(OK, Json(resources)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_bot_storage_capacity(State(app): State<App>, Path(id): Path<BotId>) -> Response {
  app
    .world(|world| world.cheat_get_bot_storage_capacity(&id))
    .map_ok(|capacity| res!(OK, Json(capacity)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_precursor_resources(
  State(app): State<App>,
  Path(id): Path<PrecursorId>,
) -> Response {
  app
    .world(|world| world.cheat_get_precursor_resources(id))
    .map_ok(|resources| res!(OK, Json(resources)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_precursor_storage_capacity(
  State(app): State<App>,
  Path(id): Path<PrecursorId>,
) -> Response {
  app
    .world(|world| world.cheat_get_precursor_storage_capacity(id))
    .map_ok(|capacity| res!(OK, Json(capacity)))
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
