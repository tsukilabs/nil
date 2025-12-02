// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::response::from_core_err;
use crate::state::App;
use crate::{bail_not_owned_by, bail_not_pending, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_core::infrastructure::building::prefecture::PrefectureBuildCatalog;
use nil_payload::infrastructure::prefecture::{
  AddPrefectureBuildOrderRequest,
  CancelPrefectureBuildOrderRequest,
  GetPrefectureBuildCatalogRequest,
};

pub async fn add_build_order(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<AddPrefectureBuildOrderRequest>,
) -> Response {
  let result = try {
    let mut world = app.world.write().await;
    bail_not_pending!(world, &player.0);
    bail_not_owned_by!(world, &player.0, req.request.coord);
    world.add_prefecture_build_order(&req.request)?;
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}

pub async fn cancel_build_order(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<CancelPrefectureBuildOrderRequest>,
) -> Response {
  let result = try {
    let mut world = app.world.write().await;
    bail_not_pending!(world, &player.0);
    bail_not_owned_by!(world, &player.0, req.coord);
    world.cancel_prefecture_build_order(req.coord)?;
  };

  result
    .map(|()| res!(OK))
    .unwrap_or_else(from_core_err)
}

pub async fn get_build_catalog(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetPrefectureBuildCatalogRequest>,
) -> Response {
  let result = try {
    let world = app.world.read().await;
    bail_not_owned_by!(world, &player.0, req.coord);
    let infra = world.city(req.coord)?.infrastructure();
    let stats = world.stats().infrastructure();
    PrefectureBuildCatalog::new(infra, &stats)?
  };

  result
    .map(|catalog| res!(OK, Json(catalog)))
    .unwrap_or_else(from_core_err)
}
