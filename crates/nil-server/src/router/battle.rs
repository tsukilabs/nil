// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::battle::*;

pub async fn simulate(State(app): State<App>, Json(req): Json<SimulateBattleRequest>) -> Response {
  app
    .world(req.world, |world| {
      world.simulate_battle(&req.attacker, &req.defender, req.luck, req.wall)
    })
    .await
    .try_map_left(|result| res!(OK, Json(result)))
    .into_inner()
}
