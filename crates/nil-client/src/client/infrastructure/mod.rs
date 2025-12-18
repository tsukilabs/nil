// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod academy;
mod prefecture;
mod stable;

use crate::client::Client;
use crate::error::Result;
use nil_payload::infrastructure::*;

impl Client {
  pub async fn toggle_building(&self, req: ToggleBuildingRequest) -> Result<()> {
    self.http.post("toggle-building", req).await
  }
}
