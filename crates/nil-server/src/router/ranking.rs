// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::FutureExt;
use nil_core::ruler::Ruler;

pub async fn get(State(app): State<App>) -> Response {
  app
    .ranking(Clone::clone)
    .map(|ranking| res!(OK, Json(ranking)))
    .await
}

pub async fn get_rank(State(app): State<App>, Json(id): Json<Ruler>) -> Response {
  app
    .ranking(|ranking| ranking.get(&id).cloned())
    .map(|rank| res!(OK, Json(rank)))
    .await
}
