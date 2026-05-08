// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use axum::extract::State;
use axum::response::Response;
use nil_payload::response::server::*;

pub async fn kind(State(app): State<App>) -> Response {
  res!(OK, GetServerKindResponse(app.server_kind()))
}

pub async fn version() -> Response {
  let version = env!("CARGO_PKG_VERSION").to_string();
  res!(OK, VersionResponse(version))
}
