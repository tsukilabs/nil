// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::ruler::Ruler;
use nil_payload::request::market::*;
use nil_payload::response::market::*;

pub async fn fee(State(app): State<App>, Json(req): Json<GetMarketFeeRequest>) -> Response {
  app
    .world(req.world, |world| world.market().fee())
    .await
    .map_left(|fee| res!(OK, GetMarketFeeResponse(fee)))
    .into_inner()
}

pub async fn get(State(app): State<App>, Json(req): Json<GetMarketRequest>) -> Response {
  app
    .world(req.world, |world| world.market().clone())
    .await
    .map_left(|market| res!(OK, GetMarketResponse(market)))
    .into_inner()
}

pub async fn send_resources(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<SendResourcesRequest>,
) -> Response {
  let sender = Ruler::from(player.0);
  app
    .world_mut(req.world, move |world| {
      world.send_resources(&sender, &req.recipient, req.resources)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
