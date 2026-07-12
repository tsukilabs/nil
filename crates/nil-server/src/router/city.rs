// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::CoreError;
use crate::middleware::authorization::CurrentPlayer;
use crate::response::{EitherExt, from_err};
use crate::{bail_if_city_is_not_owned_by, bail_if_max_chars_exceeded, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use itertools::Itertools;
use nil_core::city::PublicCity;
use nil_payload::request::city::*;
use nil_payload::response::city::*;
use tap::Pipe;

pub async fn get_cities(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetCitiesRequest>,
) -> Response {
  if req.coords.is_empty() && !req.all {
    return res!(OK, GetCitiesResponse(Vec::new()));
  }

  app
    .world(req.world, move |world| {
      world
        .continent()
        .cities_of(player.0)
        .filter(|city| req.all || req.coords.contains(&city.coord()))
        .map(|city| {
          GetCityResponse::builder(world, city.coord())
            .score(req.score)
            .build()
        })
        .try_collect()
    })
    .await
    .try_map_left(|responses| res!(OK, GetCitiesResponse(responses)))
    .into_inner()
}

pub async fn get_city(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetCityRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let world = world.read().await;
        bail_if_city_is_not_owned_by!(world, &player.0, req.coord);
        GetCityResponse::builder(&world, req.coord)
          .score(req.score)
          .build()?
      };

      result
        .map(|response| res!(OK, response))
        .unwrap_or_else(from_err)
    }
    Err(err) => from_err(err),
  }
}

pub async fn get_public_cities(
  State(app): State<App>,
  Json(req): Json<GetPublicCitiesRequest>,
) -> Response {
  if req.coords.is_empty() && !req.all {
    return res!(OK, GetPublicCitiesResponse(Vec::new()));
  }

  app
    .world(req.world, |world| {
      world
        .continent()
        .cities_by(|city| req.all || req.coords.contains(&city.coord()))
        .map(|city| {
          GetPublicCityResponse::builder(world, city.coord())
            .score(req.score)
            .build()
        })
        .try_collect()
    })
    .await
    .try_map_left(|responses| res!(OK, GetPublicCitiesResponse(responses)))
    .into_inner()
}

pub async fn get_public_city(
  State(app): State<App>,
  Json(req): Json<GetPublicCityRequest>,
) -> Response {
  app
    .world(req.world, |world| {
      GetPublicCityResponse::builder(world, req.coord)
        .score(req.score)
        .build()
    })
    .await
    .try_map_left(|response| res!(OK, response))
    .into_inner()
}

pub async fn get_city_score(
  State(app): State<App>,
  Json(req): Json<GetCityScoreRequest>,
) -> Response {
  app
    .world(req.world, |world| world.get_city_score(req.coord))
    .await
    .try_map_left(|score| res!(OK, GetCityScoreResponse(score)))
    .into_inner()
}

pub async fn rename_city(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<RenameCityRequest>,
) -> Response {
  bail_if_max_chars_exceeded!(req.name, 50);
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let mut world = world.write().await;
        bail_if_city_is_not_owned_by!(world, &player.0, req.coord);
        world.rename_city(req.coord, &req.name)?;
      };

      result
        .map(|()| res!(OK))
        .unwrap_or_else(from_err)
    }
    Err(err) => from_err(err),
  }
}

pub async fn search_city(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<SearchCityRequest>,
) -> Response {
  app
    .world_blocking(req.world, move |world| {
      world
        .continent()
        .search(req.search)?
        .into_iter()
        .filter(|city| city.is_owned_by_player_and(|it| it == &player.0))
        .cloned()
        .collect_vec()
        .pipe(Ok::<_, CoreError>)
    })
    .await
    .try_map_left(|cities| res!(OK, SearchCityResponse(cities)))
    .into_inner()
}

pub async fn search_public_city(
  State(app): State<App>,
  Json(req): Json<SearchPublicCityRequest>,
) -> Response {
  app
    .world_blocking(req.world, move |world| {
      world
        .continent()
        .search(req.search)?
        .into_iter()
        .map_into::<PublicCity>()
        .collect_vec()
        .pipe(Ok::<_, CoreError>)
    })
    .await
    .try_map_left(|cities| res!(OK, SearchPublicCityResponse(cities)))
    .into_inner()
}
