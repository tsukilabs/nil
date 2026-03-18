// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use crate::server::local;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_payload::world::*;
use tokio::task::spawn;

pub async fn save(State(app): State<App>, Json(req): Json<SaveLocalWorldRequest>) -> Response {
  if app.server_kind().is_local() {
    let f = move |bytes| {
      spawn(local::save(req.path, bytes));
    };

    app
      .world_mut(req.world, move |world| world.save(f))
      .await
      .map_left(|()| res!(OK))
      .into_inner()
  } else {
    res!(FORBIDDEN)
  }
}
