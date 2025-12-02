// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod academy;
pub mod prefecture;
pub mod prelude;
pub mod stable;

use crate::middleware::CurrentPlayer;
use crate::response::from_core_err;
use crate::state::App;
use crate::{bail_not_owned_by, bail_not_pending, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_payload::infrastructure::ToggleBuildingRequest;

pub async fn toggle(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<ToggleBuildingRequest>,
) -> Response {
  let result = try {
    let mut world = app.world.write().await;
    bail_not_pending!(world, &player.0);
    bail_not_owned_by!(world, &player.0, req.coord);
    world.toggle_building(req.coord, req.id, req.enabled)?;
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}
