// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::request::battle::*;
use nil_payload::response::battle::*;

pub async fn simulate(State(app): State<App>, Json(req): Json<SimulateBattleRequest>) -> Response {
  app
    .world_blocking(req.world, move |world| {
      world.simulate_battle(&req.attacker, &req.defender, req.luck, req.wall)
    })
    .await
    .try_map_left(|result| res!(OK, SimulateBattleResponse(result)))
    .into_inner()
}
