// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::{CoreError, Error};
use crate::middleware::authorization::CurrentPlayer;
use crate::response::{EitherExt, from_core_err};
use crate::{bail_if_city_is_not_owned_by, res};
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use itertools::Itertools;
use nil_core::city::{City, PublicCity};
use nil_payload::city::*;
use tap::Pipe;

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
        world.city(req.coord)?.clone()
      };

      result
        .map(|city| res!(OK, Json(city)))
        .unwrap_or_else(from_core_err)
    }
    Err(err) => Response::from(err),
  }
}

pub async fn get_public_cities(
  State(app): State<App>,
  Json(req): Json<GetPublicCitiesRequest>,
) -> Response {
  if req.coords.is_empty() && !req.all {
    return res!(OK, Json(Vec::<()>::new()));
  }

  app
    .world(req.world, |world| {
      let mut responses = Vec::with_capacity(req.coords.len());
      let cities: Box<dyn Iterator<Item = &City>> = if req.all {
        world.continent().cities().pipe(Box::new)
      } else {
        world
          .continent()
          .cities_by(|city| req.coords.contains(&city.coord()))
          .pipe(Box::new)
      };

      for city in cities {
        GetPublicCityResponse::builder(world, city.coord())
          .score(req.score)
          .build()?
          .pipe(|it| responses.push(it));
      }

      Ok::<_, Error>(responses)
    })
    .await
    .try_map_left(|responses| res!(OK, Json(responses)))
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
    .try_map_left(|response| res!(OK, Json(response)))
    .into_inner()
}

pub async fn get_city_score(
  State(app): State<App>,
  Json(req): Json<GetCityScoreRequest>,
) -> Response {
  app
    .world(req.world, |world| world.get_city_score(req.coord))
    .await
    .try_map_left(|score| res!(OK, Json(score)))
    .into_inner()
}

pub async fn rename_city(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<RenameCityRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let mut world = world.write().await;
        bail_if_city_is_not_owned_by!(world, &player.0, req.coord);
        world.rename_city(req.coord, &req.name)?;
      };

      result
        .map(|city| res!(OK, Json(city)))
        .unwrap_or_else(from_core_err)
    }
    Err(err) => Response::from(err),
  }
}

pub async fn search_city(State(app): State<App>, Json(req): Json<SearchCityRequest>) -> Response {
  app
    .world_blocking(req.world, move |world| {
      world
        .continent()
        .search(req.search)?
        .into_iter()
        .cloned()
        .collect_vec()
        .pipe(Ok::<_, CoreError>)
    })
    .await
    .try_map_left(|cities| res!(OK, Json(cities)))
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
    .try_map_left(|cities| res!(OK, Json(cities)))
    .into_inner()
}
