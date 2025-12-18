// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::military::maneuver::ManeuverId;
use nil_payload::military::*;

impl Client {
  pub async fn request_maneuver(&self, req: RequestManeuverRequest) -> Result<ManeuverId> {
    self
      .http
      .json_post("request-maneuver", req)
      .await
  }
}
