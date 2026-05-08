// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::continent::*;
use nil_payload::response::continent::*;

impl Client {
  pub async fn get_continent_size(
    &self,
    req: GetContinentSizeRequest,
  ) -> Result<GetContinentSizeResponse> {
    http::json_put("get-continent-size")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_public_field(
    &self,
    req: GetPublicFieldRequest,
  ) -> Result<GetPublicFieldResponse> {
    http::json_put("get-public-field")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_public_fields(
    &self,
    req: GetPublicFieldsRequest,
  ) -> Result<GetPublicFieldsResponse> {
    http::json_put("get-public-fields")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
