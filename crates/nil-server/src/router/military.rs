// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::CoreError;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_payload::military::*;

pub async fn request_maneuver(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<RequestManeuverRequest>,
) -> Response {
  app
    .world_blocking_mut(req.world, move |world| {
      if world.round().is_waiting_player(&player.0) {
        world.request_maneuver(req.request)
      } else {
        Err(CoreError::NotWaitingPlayer(player.0))
      }
    })
    .await
    .try_map_left(|result| res!(OK, Json(result)))
    .into_inner()
}
