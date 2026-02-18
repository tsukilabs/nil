// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::npc::bot::BotId;
use nil_core::npc::precursor::PrecursorId;
use nil_core::player::PlayerId;
use nil_core::world::config::{WorldConfig, WorldId};
use nil_core::world::stats::WorldStats;
use nil_payload::world::*;
use nil_server_types::RemoteWorld;

impl Client {
  pub async fn create_remote_world(&self, req: CreateRemoteWorldRequest) -> Result<WorldId> {
    http::json_post("create-remote-world")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_remote_world(&self, req: GetRemoteWorldRequest) -> Result<RemoteWorld> {
    http::json_post("get-remote-world")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_remote_worlds(&self) -> Result<Vec<RemoteWorld>> {
    http::json_get("get-remote-worlds")
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_world_bots(&self, req: GetWorldBotsRequest) -> Result<Vec<BotId>> {
    http::json_post("get-world-bots")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_world_config(&self, req: GetWorldConfigRequest) -> Result<WorldConfig> {
    http::json_post("get-world-config")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_world_players(&self, req: GetWorldPlayersRequest) -> Result<Vec<PlayerId>> {
    http::json_post("get-world-players")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_world_precursors(
    &self,
    req: GetWorldPrecursorsRequest,
  ) -> Result<Vec<PrecursorId>> {
    http::json_post("get-world-precursors")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_world_stats(&self, req: GetWorldStatsRequest) -> Result<WorldStats> {
    http::json_post("get-world-stats")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub(super) async fn leave(&self, req: LeaveRequest) -> Result<()> {
    if self.server.is_local() {
      http::post("leave")
        .body(req)
        .server(self.server)
        .maybe_authorization(self.authorization.as_ref())
        .user_agent(&self.user_agent)
        .send()
        .await
    } else {
      Ok(())
    }
  }

  pub async fn save_local_world(&self, req: SaveLocalWorldRequest) -> Result<()> {
    if self.server.is_local() {
      http::post("save-local-world")
        .body(req)
        .server(self.server)
        .maybe_authorization(self.authorization.as_ref())
        .user_agent(&self.user_agent)
        .send()
        .await
    } else {
      Ok(())
    }
  }
}
