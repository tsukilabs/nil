// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use itertools::Itertools;
use nil_core::npc::precursor::PublicPrecursor;
use nil_payload::npc::precursor::*;

pub async fn get_coords(
  State(app): State<App>,
  Json(req): Json<GetPrecursorCoordsRequest>,
) -> Response {
  app
    .continent(req.world, |k| k.coords_of(req.id).collect_vec())
    .await
    .map_left(|coords| res!(OK, Json(coords)))
    .into_inner()
}

pub async fn get_public(
  State(app): State<App>,
  Json(req): Json<GetPublicPrecursorRequest>,
) -> Response {
  app
    .precursor_manager(req.world, |pm| PublicPrecursor::new(pm.precursor(req.id)))
    .await
    .map_left(|precursor| res!(OK, Json(precursor)))
    .into_inner()
}
