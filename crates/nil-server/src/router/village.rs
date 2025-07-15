// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::CoreResult;
use crate::middleware::CurrentPlayer;
use crate::response::from_core_err;
use crate::state::App;
use crate::{bail_not_owned_by, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::village::{Coord, Village};

pub async fn get(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(coord): Json<Coord>,
) -> Response {
  let result: CoreResult<Village> = try {
    let world = app.world.read().await;
    bail_not_owned_by!(world, &player.0, coord);
    world.village(coord)?.clone()
  };

  result
    .map(|village| res!(OK, Json(village)))
    .unwrap_or_else(from_core_err)
}

pub async fn rename(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json((coord, name)): Json<(Coord, String)>,
) -> Response {
  let result: CoreResult<()> = try {
    let mut world = app.world.write().await;
    bail_not_owned_by!(world, &player.0, coord);
    world.rename_village(coord, &name)?;
  };

  result
    .map(|village| res!(OK, Json(village)))
    .unwrap_or_else(from_core_err)
}
