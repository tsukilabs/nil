// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::app::App;
use crate::error::{Error, Result};
use crate::middleware::authorization::{decode_jwt, encode_jwt};
use crate::res;
use crate::response::from_err;
use axum::extract::{Json, State};
use axum::response::Response;
use nil_core::player::PlayerId;
use nil_payload::{AuthorizeRequest, ValidateTokenRequest};
use nil_server_types::ServerKind;
use nil_server_types::auth::Token;

pub async fn authorize(State(app): State<App>, Json(req): Json<AuthorizeRequest>) -> Response {
  let result = try bikeshed Result<Token> {
    match app.server_kind() {
      ServerKind::Local { .. } => encode_jwt(req.player).await?,
      ServerKind::Remote => {
        let Some(password) = req.password else {
          return from_err(Error::MissingPassword);
        };

        if app
          .database()
          .get_user(&req.player)
          .await?
          .verify_password(password)
          .await?
        {
          encode_jwt(req.player).await?
        } else {
          return from_err(Error::IncorrectUserCredentials);
        }
      }
    }
  };

  result
    .map(|token| res!(OK, Json(token)))
    .unwrap_or_else(from_err)
}

pub async fn validate_token(
  State(app): State<App>,
  Json(req): Json<ValidateTokenRequest>,
) -> Response {
  let player = decode_jwt(req.token)
    .await
    .map(|token| token.claims.sub)
    .ok();

  if app.server_kind().is_remote()
    && let Some(player) = player.clone()
  {
    match app.database().user_exists(player).await {
      Ok(true) => {}
      Ok(false) => return res!(OK, Json(None::<PlayerId>)),
      Err(err) => return from_err(err),
    }
  }

  res!(OK, Json(player))
}
