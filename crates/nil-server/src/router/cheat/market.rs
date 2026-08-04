// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_core::world::cheat;
use nil_payload::request::cheat::market::*;

pub async fn set_fee(
  State(app): State<App>,
  Json(req): Json<CheatSetMarketFeeRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| cheat::set_market_fee(world, req.fee))
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}

pub async fn set_vault_resources(
  State(app): State<App>,
  Json(req): Json<CheatSetMarketVaultResourcesRequest>,
) -> Response {
  app
    .world_mut(req.world, |world| {
      cheat::set_market_vault_resources(world, req.resources)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
