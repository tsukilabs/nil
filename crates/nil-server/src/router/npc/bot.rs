// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use itertools::Itertools;
use nil_core::npc::bot::PublicBot;
use nil_payload::npc::bot::{GetBotCoordsRequest, GetPublicBotRequest};

pub async fn get_coords(State(app): State<App>, Json(req): Json<GetBotCoordsRequest>) -> Response {
  app
    .continent(|k| k.coords_of(req.id).collect_vec())
    .map(|coords| res!(OK, Json(coords)))
    .await
}

pub async fn get_public(State(app): State<App>, Json(req): Json<GetPublicBotRequest>) -> Response {
  app
    .bot_manager(|bm| bm.bot(&req.id).map(PublicBot::from))
    .map_ok(|bot| res!(OK, Json(bot)))
    .unwrap_or_else(from_core_err)
    .await
}
