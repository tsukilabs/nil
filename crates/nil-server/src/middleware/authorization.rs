// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::AnyResult;
use crate::res;
use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use derive_more::{Deref, From, Into};
use jiff::{SignedDuration, Zoned};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use nil_core::player::PlayerId;
use nil_core::ruler::Ruler;
use nil_server_types::Token;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::LazyLock;

// Using a known secret is not a problem for local servers.
static JWT_SECRET: LazyLock<Box<str>> = LazyLock::new(|| {
  env::var("NIL_JWT_SECRET")
    .map(Box::from)
    .unwrap_or_else(|_| Box::from("CALL-OF-NIL"))
});

pub async fn authorization(mut request: Request, next: Next) -> Response {
  if let Some(header) = request.headers().get(AUTHORIZATION)
    && let Ok(token) = header.to_str().map(Token::new)
    && let Ok(data) = decode_jwt(&token)
  {
    request
      .extensions_mut()
      .insert(CurrentPlayer(data.claims.sub));

    next.run(request).await
  } else {
    res!(UNAUTHORIZED)
  }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
  pub sub: PlayerId,
  pub exp: usize,
  pub iat: usize,
}

pub(crate) fn encode_jwt(player: PlayerId) -> AnyResult<Token> {
  let now = Zoned::now();
  let iat = now.timestamp().as_millisecond().try_into()?;
  let exp = now
    .saturating_add(SignedDuration::from_hours(300))
    .timestamp()
    .as_millisecond()
    .try_into()?;

  let token = encode(
    &Header::default(),
    &Claims { sub: player, iat, exp },
    &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
  )?;

  Ok(Token::new(token))
}

pub(crate) fn decode_jwt(token: &Token) -> AnyResult<TokenData<Claims>> {
  let claims = decode(
    token,
    &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
    &Validation::default(),
  )?;

  Ok(claims)
}

#[derive(Clone, Debug, Deref, From, Into, PartialEq, Eq)]
pub struct CurrentPlayer(pub(crate) PlayerId);

impl From<CurrentPlayer> for Ruler {
  fn from(player: CurrentPlayer) -> Self {
    Ruler::Player { id: player.0 }
  }
}

impl PartialEq<PlayerId> for CurrentPlayer {
  fn eq(&self, other: &PlayerId) -> bool {
    self.0.eq(other)
  }
}
