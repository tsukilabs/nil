// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::city::Stability;
use nil_core::continent::Coord;

pub async fn set_stability(
  State(app): State<App>,
  Json((coord, stability)): Json<(Coord, Stability)>,
) -> Response {
  app
    .world_mut(|world| world.cheat_set_stability(coord, stability))
    .map_ok(|()| res!(OK))
    .unwrap_or_else(from_core_err)
    .await
}
