// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::world::World;
use nil_payload::round::*;

pub async fn get(State(app): State<App>, Json(req): Json<GetRoundRequest>) -> Response {
  app
    .round(req.world, Clone::clone)
    .await
    .map_left(|round| res!(OK, Json(round)))
    .into_inner()
}

pub async fn set_ready(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<SetPlayerReadyRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      world.set_player_ready(&player, req.is_ready)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn start(State(app): State<App>, Json(req): Json<StartRoundRequest>) -> Response {
  app
    .world_mut(req.world, World::start_round)
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
