// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::EitherExt;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use itertools::Itertools;
use nil_core::npc::bot::PublicBot;
use nil_payload::npc::bot::*;

pub async fn get_coords(State(app): State<App>, Json(req): Json<GetBotCoordsRequest>) -> Response {
  app
    .continent(req.world, |k| k.coords_of(req.id).collect_vec())
    .await
    .map_left(|coords| res!(OK, Json(coords)))
    .into_inner()
}

pub async fn get_public(State(app): State<App>, Json(req): Json<GetPublicBotRequest>) -> Response {
  app
    .bot_manager(req.world, |bm| bm.bot(&req.id).map(PublicBot::from))
    .await
    .try_map_left(|bot| res!(OK, Json(bot)))
    .into_inner()
}
