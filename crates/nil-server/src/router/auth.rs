// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::Error;
use crate::middleware::authorization::{decode_jwt, encode_jwt};
use crate::res;
use crate::response::from_err;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_core::player::PlayerId;
use nil_payload::{AuthorizeRequest, ValidateTokenRequest};
use nil_server_types::ServerKind;
use tokio::task::spawn_blocking;

pub async fn authorize(State(app): State<App>, Json(req): Json<AuthorizeRequest>) -> Response {
  let Ok(result) = spawn_blocking(move || {
    match app.server_kind() {
      ServerKind::Local { .. } => encode_jwt(req.player),
      ServerKind::Remote => {
        let Some(password) = req.password else {
          return Err(Error::MissingPassword);
        };

        if app
          .database()
          .get_user(&req.player)
          .map_err(Into::<Error>::into)?
          .verify_password(&password)
        {
          encode_jwt(req.player)
        } else {
          Err(Error::IncorrectUserCredentials)
        }
      }
    }
  })
  .await
  else {
    return res!(INTERNAL_SERVER_ERROR);
  };

  result
    .map(|token| res!(OK, Json(token)))
    .unwrap_or_else(Response::from)
}

pub async fn validate_token(
  State(app): State<App>,
  Json(req): Json<ValidateTokenRequest>,
) -> Response {
  let Ok(player) = spawn_blocking(move || {
    decode_jwt(&req.token)
      .map(|token| token.claims.sub)
      .ok()
  })
  .await
  else {
    return res!(INTERNAL_SERVER_ERROR);
  };

  if app.server_kind().is_remote()
    && let Some(player) = player.clone()
  {
    match app.database().user_exists(player) {
      Ok(true) => {}
      Ok(false) => return res!(OK, Json(None::<PlayerId>)),
      Err(err) => return from_err(err),
    }
  }

  res!(OK, Json(player))
}
