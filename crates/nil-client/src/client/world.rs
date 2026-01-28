// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::world::{WorldConfig, WorldStats};
use nil_payload::world::*;

impl Client {
  pub async fn get_remote_world(
    &self,
    req: GetRemoteWorldRequest,
  ) -> Result<GetRemoteWorldResponse> {
    http::json_post("get-remote-world")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn get_remote_worlds(&self) -> Result<Vec<GetRemoteWorldResponse>> {
    http::json_get("get-remote-worlds")
      .server(self.server)
      .send()
      .await
  }

  pub async fn get_world_config(&self, req: GetWorldConfigRequest) -> Result<WorldConfig> {
    http::json_post("get-world-config")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn get_world_stats(&self, req: GetWorldStatsRequest) -> Result<WorldStats> {
    http::json_post("get-world-stats")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn save_local_world(&self, req: SaveLocalWorldRequest) -> Result<()> {
    http::post("save-local-world")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_deref())
      .send()
      .await
  }
}
