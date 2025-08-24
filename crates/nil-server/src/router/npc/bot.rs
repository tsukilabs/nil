// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, Path, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use itertools::Itertools;
use nil_core::npc::bot::{BotId, PublicBot};

pub async fn get_coords(State(app): State<App>, Path(id): Path<BotId>) -> Response {
  app
    .continent(|k| k.coords_of(id).collect_vec())
    .map(|coords| res!(OK, Json(coords)))
    .await
}

pub async fn get_public(State(app): State<App>, Path(id): Path<BotId>) -> Response {
  app
    .bot_manager(|bm| bm.bot(&id).map(PublicBot::from))
    .map_ok(|bot| res!(OK, Json(bot)))
    .unwrap_or_else(from_core_err)
    .await
}
