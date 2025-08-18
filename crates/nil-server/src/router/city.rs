// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::CoreResult;
use crate::middleware::CurrentPlayer;
use crate::response::from_core_err;
use crate::state::App;
use crate::{bail_not_owned_by, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use itertools::Itertools;
use nil_core::city::{City, PublicCity};
use nil_core::continent::Coord;

pub async fn get(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(coord): Json<Coord>,
) -> Response {
  let result: CoreResult<City> = try {
    let world = app.world.read().await;
    bail_not_owned_by!(world, &player.0, coord);
    world.city(coord)?.clone()
  };

  result
    .map(|city| res!(OK, Json(city)))
    .unwrap_or_else(from_core_err)
}

pub async fn get_all_public(State(app): State<App>) -> Response {
  app
    .continent(|k| {
      k.cities()
        .map(PublicCity::from)
        .collect_vec()
    })
    .map(|cities| res!(OK, Json(cities)))
    .await
}

pub async fn get_public(State(app): State<App>, Json(coord): Json<Coord>) -> Response {
  app
    .continent(|k| k.city(coord).map(PublicCity::from))
    .map_ok(|city| res!(OK, Json(city)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_public_by(State(app): State<App>, Json(coords): Json<Vec<Coord>>) -> Response {
  app
    .continent(|k| {
      k.cities_by(|city| coords.contains(&city.coord()))
        .map(PublicCity::from)
        .collect_vec()
    })
    .map(|cities| res!(OK, Json(cities)))
    .await
}

pub async fn rename(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json((coord, name)): Json<(Coord, String)>,
) -> Response {
  let result: CoreResult<()> = try {
    let mut world = app.world.write().await;
    bail_not_owned_by!(world, &player.0, coord);
    world.rename_city(coord, &name)?;
  };

  result
    .map(|city| res!(OK, Json(city)))
    .unwrap_or_else(from_core_err)
}
