// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::response::from_core_err;
use crate::state::App;
use crate::{bail_not_owned_by, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::TryFutureExt;
use nil_core::city::PublicCity;
use nil_payload::city::{
  GetCityRequest,
  GetCityScoreRequest,
  GetPublicCityRequest,
  RenameCityRequest,
};

pub async fn get(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetCityRequest>,
) -> Response {
  let result = try {
    let world = app.world.read().await;
    bail_not_owned_by!(world, &player.0, req.coord);
    world.city(req.coord)?.clone()
  };

  result
    .map(|city| res!(OK, Json(city)))
    .unwrap_or_else(from_core_err)
}

pub async fn get_public(State(app): State<App>, Json(req): Json<GetPublicCityRequest>) -> Response {
  app
    .continent(|k| k.city(req.coord).map(PublicCity::from))
    .map_ok(|city| res!(OK, Json(city)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_score(State(app): State<App>, Json(req): Json<GetCityScoreRequest>) -> Response {
  app
    .world(|world| world.get_city_score(req.coord))
    .map_ok(|score| res!(OK, Json(score)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn rename(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<RenameCityRequest>,
) -> Response {
  let result = try {
    let mut world = app.world.write().await;
    bail_not_owned_by!(world, &player.0, req.coord);
    world.rename_city(req.coord, &req.name)?;
  };

  result
    .map(|city| res!(OK, Json(city)))
    .unwrap_or_else(from_core_err)
}
