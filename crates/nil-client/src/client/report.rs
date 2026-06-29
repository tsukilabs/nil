// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::report::*;

impl Client {
  pub async fn forward_report(&self, req: ForwardReportRequest) -> Result<()> {
    http::post("forward-report")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
