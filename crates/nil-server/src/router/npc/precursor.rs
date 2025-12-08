// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::state::App;
use axum::extract::{Json, State};
use axum::response::Response;
use futures::FutureExt;
use itertools::Itertools;
use nil_core::npc::precursor::PublicPrecursor;
use nil_payload::npc::precursor::{GetPrecursorCoordsRequest, GetPublicPrecursorRequest};

pub async fn get_coords(
  State(app): State<App>,
  Json(req): Json<GetPrecursorCoordsRequest>,
) -> Response {
  app
    .continent(|k| k.coords_of(req.id).collect_vec())
    .map(|coords| res!(OK, Json(coords)))
    .await
}

pub async fn get_public(
  State(app): State<App>,
  Json(req): Json<GetPublicPrecursorRequest>,
) -> Response {
  app
    .precursor_manager(|pm| PublicPrecursor::new(pm.precursor(req.id)))
    .map(|precursor| res!(OK, Json(precursor)))
    .await
}
