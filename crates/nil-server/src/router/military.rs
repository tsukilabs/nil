// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::response::from_core_err;
use crate::state::App;
use crate::{bail_not_pending, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_payload::military::*;

pub async fn request_maneuver(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<RequestManeuverRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let mut world = world.write().await;
        bail_not_pending!(world, &player.0);
        world.request_maneuver(req.request)?
      };

      result
        .map(|id| res!(OK, Json(id)))
        .unwrap_or_else(from_core_err)
    }
    Err(err) => Response::from(err),
  }
}
