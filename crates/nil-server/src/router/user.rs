// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use crate::response::from_database_err;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_database::model::user::NewUser;
use nil_payload::user::*;

pub async fn create(State(app): State<App>, Json(req): Json<CreateUserRequest>) -> Response {
  if app.server_kind().is_remote() {
    let result = try {
      let new = NewUser::new(req.player, &req.password)?;
      app.database().create_user(&new)?;
    };

    result
      .map(|()| res!(CREATED))
      .unwrap_or_else(from_database_err)
  } else {
    res!(FORBIDDEN)
  }
}

pub async fn exists(State(app): State<App>, Json(req): Json<UserExistsRequest>) -> Response {
  if app.server_kind().is_remote() {
    app
      .database()
      .user_exists(req.user)
      .map(|exists| res!(OK, Json(exists)))
      .unwrap_or_else(from_database_err)
  } else {
    res!(FORBIDDEN)
  }
}
