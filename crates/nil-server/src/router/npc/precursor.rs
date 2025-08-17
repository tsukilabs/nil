// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::res;
use crate::state::App;
use axum::extract::{Json, Path, State};
use axum::response::Response;
use futures::FutureExt;
use itertools::Itertools;
use nil_core::npc::precursor::{PrecursorId, PublicPrecursor};

pub async fn get_coords(State(app): State<App>, Path(id): Path<PrecursorId>) -> Response {
  app
    .continent(|k| {
      k.precursor_coords_by(|precursor| precursor == id)
        .collect_vec()
    })
    .map(|coords| res!(OK, Json(coords)))
    .await
}

pub async fn get_public(State(app): State<App>, Path(id): Path<PrecursorId>) -> Response {
  app
    .precursor_manager(|pm| PublicPrecursor::new(pm.precursor(id)))
    .map(|precursor| res!(OK, Json(precursor)))
    .await
}
