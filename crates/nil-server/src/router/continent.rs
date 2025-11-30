// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::CoreResult;
use crate::res;
use crate::response::from_core_err;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::{FutureExt, TryFutureExt};
use nil_core::continent::{Continent, Coord, PublicField};
use nil_payload::continent::{GetPublicFieldRequest, GetPublicFieldsRequest};

pub async fn get_field(State(app): State<App>, Json(req): Json<GetPublicFieldRequest>) -> Response {
  app
    .continent(|k| k.field(req.coord).map(PublicField::from))
    .map_ok(|field| res!(OK, Json(field)))
    .unwrap_or_else(from_core_err)
    .await
}

pub async fn get_fields(
  State(app): State<App>,
  Json(req): Json<GetPublicFieldsRequest>,
) -> Response {
  let result: CoreResult<Vec<(Coord, PublicField)>> = try {
    let world = app.world.read().await;
    let continent = world.continent();
    let mut fields = Vec::with_capacity(req.coords.len());

    for coord in req.coords {
      let field = continent.field(coord)?;
      fields.push((coord, PublicField::from(field)));
    }

    fields
  };

  result
    .map(|fields| res!(OK, Json(fields)))
    .unwrap_or_else(from_core_err)
}

pub async fn size(State(app): State<App>) -> Response {
  app
    .continent(Continent::size)
    .map(|size| res!(OK, Json(size)))
    .await
}
