// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::response::{EitherExt, from_core_err};
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_core::continent::{Continent, PublicField};
use nil_payload::continent::*;

pub async fn get_field(State(app): State<App>, Json(req): Json<GetPublicFieldRequest>) -> Response {
  app
    .continent(req.world, |k| k.field(req.coord).map(PublicField::from))
    .await
    .try_map_left(|field| res!(OK, Json(field)))
    .into_inner()
}

pub async fn get_fields(
  State(app): State<App>,
  Json(req): Json<GetPublicFieldsRequest>,
) -> Response {
  match app.get(req.world) {
    Ok(world) => {
      let result = try {
        let world = world.read().await;
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
    Err(err) => Response::from(err),
  }
}

pub async fn size(State(app): State<App>, Json(req): Json<GetContinentSizeRequest>) -> Response {
  app
    .continent(req.world, Continent::size)
    .await
    .map_left(|size| res!(OK, Json(size)))
    .into_inner()
}
