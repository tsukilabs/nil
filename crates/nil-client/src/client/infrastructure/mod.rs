// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod academy;
mod prefecture;
mod stable;

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_payload::infrastructure::*;

impl Client {
  pub async fn toggle_building(&self, req: ToggleBuildingRequest) -> Result<()> {
    http::post("toggle-building")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
