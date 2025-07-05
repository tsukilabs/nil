// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::CoreResult;
use crate::middleware::CurrentPlayer;
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::infrastructure::building::prefecture::{
  PrefectureBuildOrderOptions,
  PrefectureCatalog,
};
use nil_core::village::Coord;

pub async fn add_build_order(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(options): Json<PrefectureBuildOrderOptions>,
) -> Response {
  let result: CoreResult<()> = try {
    let mut world = app.world.write().await;
    world
      .round()
      .check_if_player_is_pending(&current_player.0)?;

    if world
      .village(options.coord)?
      .is_owned_by_player_and(|id| &current_player.0 == id)
    {
      world.add_prefecture_build_order(&options)?;
    } else {
      return res!(FORBIDDEN);
    }
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}

pub async fn cancel_build_order(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(coord): Json<Coord>,
) -> Response {
  let result: CoreResult<()> = try {
    let mut world = app.world.write().await;
    world
      .round()
      .check_if_player_is_pending(&current_player.0)?;

    if world
      .village(coord)?
      .is_owned_by_player_and(|id| &current_player.0 == id)
    {
      world.cancel_prefecture_build_order(coord)?;
    } else {
      return res!(FORBIDDEN);
    }
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}

pub async fn get_catalog(
  State(app): State<App>,
  Extension(current_player): Extension<CurrentPlayer>,
  Json(coord): Json<Coord>,
) -> Response {
  let result: CoreResult<PrefectureCatalog> = try {
    let world = app.world.read().await;
    let village = world.village(coord)?;
    if village.is_owned_by_player_and(|id| *current_player == *id) {
      let infra = village.infrastructure();
      let stats = world.stats().infrastructure();
      PrefectureCatalog::new(infra, &stats)?
    } else {
      return res!(FORBIDDEN);
    }
  };

  result
    .map(|catalog| res!(OK, Json(catalog)))
    .unwrap_or_else(from_core_err)
}
