// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::request::world::*;
use nil_payload::response::world::*;

impl Client {
  pub async fn create_remote_world(
    &self,
    req: CreateRemoteWorldRequest,
  ) -> Result<CreateRemoteWorldResponse> {
    http::json_post("create-remote-world")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn delete_remote_world(&self, req: DeleteRemoteWorldRequest) -> Result<()> {
    http::post("delete-remote-world")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_remote_world(
    &self,
    req: GetRemoteWorldRequest,
  ) -> Result<GetRemoteWorldResponse> {
    http::json_put("get-remote-world")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_remote_world_limit(&self) -> Result<GetRemoteWorldLimitResponse> {
    http::json_get("get-remote-world-limit")
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_remote_world_limit_per_user(
    &self,
  ) -> Result<GetRemoteWorldLimitPerUserResponse> {
    http::json_get("get-remote-world-limit-per-user")
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_remote_worlds(&self) -> Result<GetRemoteWorldsResponse> {
    http::json_get("get-remote-worlds")
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_world_bots(&self, req: GetWorldBotsRequest) -> Result<GetWorldBotsResponse> {
    http::json_put("get-world-bots")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_world_config(
    &self,
    req: GetWorldConfigRequest,
  ) -> Result<GetWorldConfigResponse> {
    http::json_put("get-world-config")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_world_players(
    &self,
    req: GetWorldPlayersRequest,
  ) -> Result<GetWorldPlayersResponse> {
    http::json_put("get-world-players")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_world_precursors(
    &self,
    req: GetWorldPrecursorsRequest,
  ) -> Result<GetWorldPrecursorsResponse> {
    http::json_put("get-world-precursors")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_world_stats(&self, req: GetWorldStatsRequest) -> Result<GetWorldStatsResponse> {
    http::json_put("get-world-stats")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
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
        .circuit_breaker(self.circuit_breaker())
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
        .circuit_breaker(self.circuit_breaker())
        .user_agent(&self.user_agent)
        .send()
        .await
    } else {
      Ok(())
    }
  }
}
