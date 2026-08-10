// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::world::cheat;
use nil_payload::request::cheat::capital::*;
use nil_payload::response::cheat::capital::*;

pub async fn get_influence(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatGetInfluenceRequest>,
) -> Response {
  let ruler = req.ruler.unwrap_or_else(|| player.into());
  app
    .world(req.world, move |world| cheat::get_influence(world, &ruler))
    .await
    .try_map_left(|influence| res!(OK, CheatGetInfluenceResponse(influence)))
    .into_inner()
}

pub async fn set_influence(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CheatSetInfluenceRequest>,
) -> Response {
  let ruler = req.ruler.unwrap_or_else(|| player.into());
  app
    .world_mut(req.world, move |world| {
      cheat::set_influence(world, &ruler, req.influence)
    })
    .await
    .try_map_left(|()| res!(OK))
    .into_inner()
}
