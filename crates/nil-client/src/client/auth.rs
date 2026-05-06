// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use crate::retry::Retry;
use nil_payload::request::auth::*;
use nil_payload::response::auth::*;
use std::num::NonZeroU8;

impl Client {
  pub async fn authorize(&self, req: AuthorizeRequest) -> Result<AuthorizeResponse> {
    http::json_post("authorize")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn validate_token<T>(&self, req: T) -> Result<ValidateTokenResponse>
  where
    T: Into<ValidateTokenRequest>,
  {
    let retry = Retry::builder()
      .attempts(unsafe { NonZeroU8::new_unchecked(3) })
      .backoff(false)
      .build();

    http::json_put("validate-token")
      .body(Into::<ValidateTokenRequest>::into(req))
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
