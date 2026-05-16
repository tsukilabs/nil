// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use bon::Builder;
use nil_core::player::PlayerId;
use nil_crypto::password::Password;
use nil_server_types::auth::Token;
use serde::{Deserialize, Serialize};

#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export, optional_fields = nullable))]
pub struct AuthorizeRequest {
  #[builder(into)]
  pub player: PlayerId,
  #[serde(default)]
  #[builder(into)]
  pub password: Option<Password>,
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct ValidateTokenRequest {
  #[builder(start_fn, into)]
  pub token: Token,
}

impl<T> From<T> for ValidateTokenRequest
where
  T: AsRef<str>,
{
  fn from(token: T) -> Self {
    Self { token: Token::new(token) }
  }
}
