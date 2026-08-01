// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::request::market::*;
use nil_payload::response::market::*;

pub async fn fee(State(app): State<App>, Json(req): Json<GetMarketFeeRequest>) -> Response {
  app
    .world(req.world, |world| world.market().fee())
    .await
    .map_left(|fee| res!(OK, GetMarketFeeResponse(fee)))
    .into_inner()
}
