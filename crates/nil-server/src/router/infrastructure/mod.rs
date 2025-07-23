// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

pub mod prefecture;
pub mod prelude;

use crate::error::CoreResult;
use crate::middleware::CurrentPlayer;
use crate::response::from_core_err;
use crate::state::App;
use crate::{bail_not_owned_by, bail_not_pending, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::continent::Coord;
use nil_core::infrastructure::building::BuildingId;

pub async fn toggle(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json((coord, id, enabled)): Json<(Coord, BuildingId, bool)>,
) -> Response {
  let result: CoreResult<()> = try {
    let mut world = app.world.write().await;
    bail_not_pending!(world, &player.0);
    bail_not_owned_by!(world, &player.0, coord);
    world.toggle_building(coord, id, enabled)?;
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}
