// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::player::PlayerId;
use nil_payload::{AuthorizeRequest, ValidateTokenRequest};
use nil_server_types::Token;

impl Client {
  pub async fn authorize(&self, req: AuthorizeRequest) -> Result<Token> {
    http::json_post("authorize")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn validate_token<T>(&self, req: T) -> Result<Option<PlayerId>>
  where
    T: Into<ValidateTokenRequest>,
  {
    http::json_put("validate-token")
      .body(Into::<ValidateTokenRequest>::into(req))
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
