// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::military::maneuver::ManeuverId;
use nil_payload::military::*;

impl Client {
  pub async fn request_maneuver(&self, req: RequestManeuverRequest) -> Result<ManeuverId> {
    http::json_post("request-maneuver")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
