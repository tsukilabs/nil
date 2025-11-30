// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::world::{WorldConfig, WorldStats};
use nil_payload::world::SaveWorldRequest;

impl Client {
  /// GET `/world/config`
  pub async fn get_world_config(&self) -> Result<WorldConfig> {
    self.http.get_json("world/config").await
  }

  /// GET `/world/stats`
  pub async fn get_world_stats(&self) -> Result<WorldStats> {
    self.http.get_json("world/stats").await
  }

  /// POST `/world/save`
  pub async fn save_world(&self, req: SaveWorldRequest) -> Result<()> {
    self.http.post("world/save", req).await
  }
}
