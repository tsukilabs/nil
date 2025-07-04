// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::FutureExt;
use nil_core::continent::Continent;

pub async fn size(State(app): State<App>) -> Response {
  app
    .continent(Continent::size)
    .map(|size| res!(OK, Json(size)))
    .await
}
