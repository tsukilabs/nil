// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::middleware::authorization::CurrentPlayer;
use crate::res;
use crate::response::EitherExt;
use axum::extract::{Extension, Json, State};
use axum::response::Response;
use nil_payload::request::capital::*;
use nil_payload::response::capital::*;

pub async fn city_limit(
  State(app): State<App>,
  Extension(player): Extension<CurrentPlayer>,
  Json(req): Json<GetCityLimitRequest>,
) -> Response {
  app
    .world(req.world, move |world| world.get_city_limit(player))
    .await
    .try_map_left(|limit| res!(OK, GetCityLimitResponse(limit)))
    .into_inner()
}
