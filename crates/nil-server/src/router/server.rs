// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::{VERSION, res};
use axum::extract::State;
use axum::response::Response;
use nil_payload::response::server::*;
use semver::Version;

pub async fn kind(State(app): State<App>) -> Response {
  res!(OK, GetServerKindResponse(app.server_kind()))
}

pub async fn version() -> Response {
  match Version::parse(VERSION) {
    Ok(version) => res!(OK, VersionResponse(version)),
    Err(_) => res!(INTERNAL_SERVER_ERROR),
  }
}
