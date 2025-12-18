// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::middleware::CurrentPlayer;
use crate::response::{EitherExt, from_core_err};
use crate::state::App;
use crate::{bail_not_owned_by, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use itertools::Itertools;
use nil_core::city::PublicCity;
use nil_payload::city::*;

pub async fn get(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetCityRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let world = world.read().await;
        bail_not_owned_by!(world, &player.0, req.coord);
        world.city(req.coord)?.clone()
      };

      result
        .map(|city| res!(OK, Json(city)))
        .unwrap_or_else(from_core_err)
    }
    Err(err) => Response::from(err),
  }
}

pub async fn get_public(State(app): State<App>, Json(req): Json<GetPublicCityRequest>) -> Response {
  app
    .continent(req.world, |k| k.city(req.coord).map(PublicCity::from))
    .await
    .try_map_left(|city| res!(OK, Json(city)))
    .into_inner()
}

pub async fn get_score(State(app): State<App>, Json(req): Json<GetCityScoreRequest>) -> Response {
  app
    .world(req.world, |world| world.get_city_score(req.coord))
    .await
    .try_map_left(|score| res!(OK, Json(score)))
    .into_inner()
}

pub async fn rename(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<RenameCityRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let mut world = world.write().await;
        bail_not_owned_by!(world, &player.0, req.coord);
        world.rename_city(req.coord, &req.name)?;
      };

      result
        .map(|city| res!(OK, Json(city)))
        .unwrap_or_else(from_core_err)
    }
    Err(err) => Response::from(err),
  }
}

pub async fn search(State(app): State<App>, Json(req): Json<SearchCityRequest>) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let world = world.read().await;
        world
          .continent()
          .search(req.search)?
          .into_iter()
          .cloned()
          .collect_vec()
      };

      result
        .map(|city| res!(OK, Json(city)))
        .unwrap_or_else(from_core_err)
    }
    Err(err) => Response::from(err),
  }
}

pub async fn search_public(
  State(app): State<App>,
  Json(req): Json<SearchPublicCityRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let world = world.read().await;
        world
          .continent()
          .search(req.search)?
          .into_iter()
          .map_into::<PublicCity>()
          .collect_vec()
      };

      result
        .map(|city| res!(OK, Json(city)))
        .unwrap_or_else(from_core_err)
    }
    Err(err) => Response::from(err),
  }
}
