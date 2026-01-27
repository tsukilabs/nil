// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::world::{WorldConfig, WorldStats};
use nil_payload::world::*;

impl Client {
  pub async fn get_remote_world(
    &self,
    req: GetRemoteWorldRequest,
  ) -> Result<GetRemoteWorldResponse> {
    self
      .http
      .json_post("get-remote-world", req)
      .await
  }

  pub async fn get_remote_worlds(&self) -> Result<Vec<GetRemoteWorldResponse>> {
    self.http.json_get("get-remote-worlds").await
  }

  pub async fn get_world_config(&self, req: GetWorldConfigRequest) -> Result<WorldConfig> {
    self
      .http
      .json_post("get-world-config", req)
      .await
  }

  pub async fn get_world_stats(&self, req: GetWorldStatsRequest) -> Result<WorldStats> {
    self
      .http
      .json_post("get-world-stats", req)
      .await
  }

  pub async fn save_local_world(&self, req: SaveLocalWorldRequest) -> Result<()> {
    self.http.post("save-local-world", req).await
  }
}
