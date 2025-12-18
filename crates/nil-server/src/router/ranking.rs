// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::ranking::*;

pub async fn get(State(app): State<App>, Json(req): Json<GetRankingRequest>) -> Response {
  app
    .ranking(req.world, Clone::clone)
    .await
    .map_left(|ranking| res!(OK, Json(ranking)))
    .into_inner()
}

pub async fn get_rank(State(app): State<App>, Json(req): Json<GetRankRequest>) -> Response {
  app
    .ranking(req.world, |ranking| ranking.get(&req.ruler).cloned())
    .await
    .map_left(|rank| res!(OK, Json(rank)))
    .into_inner()
}
