// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::CoreError;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::{EitherExt, from_err};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_payload::request::military::*;
use nil_payload::response::military::*;

pub async fn get_army(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetArmyRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let army = world
          .read()
          .await
          .military()
          .army(req.id)
          .cloned()?;

        if player != *army.owner() {
          return res!(FORBIDDEN);
        }

        army
      };

      result
        .map(|army| res!(OK, GetArmyResponse(army)))
        .unwrap_or_else(from_err)
    }
    Err(err) => from_err(err),
  }
}

pub async fn get_maneuver(State(app): State<App>, Json(req): Json<GetManeuverRequest>) -> Response {
  app
    .military(req.world, |military| military.maneuver(req.id).cloned())
    .await
    .try_map_left(|maneuver| res!(OK, GetManeuverResponse(maneuver)))
    .into_inner()
}

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
    .try_map_left(|id| res!(OK, RequestManeuverResponse(id)))
    .into_inner()
}
