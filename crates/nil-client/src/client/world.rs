// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::world::{WorldConfig, WorldStats};
use nil_payload::world::SaveWorldRequest;

impl Client {
  pub async fn get_world_config(&self) -> Result<WorldConfig> {
    self.http.json_get("get-world-config").await
  }

  pub async fn get_world_stats(&self) -> Result<WorldStats> {
    self.http.json_get("get-world-stats").await
  }

  pub async fn save_world(&self, req: SaveWorldRequest) -> Result<()> {
    self.http.post("save-world", req).await
  }
}
