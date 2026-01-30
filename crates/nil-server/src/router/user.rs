// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::res;
use crate::response::from_database_err;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_database::model::user_data::NewUserData;
use nil_database::sql_types::user::User;
use nil_payload::user::*;

pub async fn create(State(app): State<App>, Json(req): Json<CreateUserRequest>) -> Response {
  if app.server_kind().is_remote() {
    let database = app.database();
    let user = User::from(req.player);
    let result = try {
      NewUserData::new(user, &req.password)?.create(&database)?;
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
    let user = User::from(req.user);
    app
      .database()
      .user_data_exists(&user)
      .map(|exists| res!(OK, Json(exists)))
      .unwrap_or_else(from_database_err)
  } else {
    res!(FORBIDDEN)
  }
}
