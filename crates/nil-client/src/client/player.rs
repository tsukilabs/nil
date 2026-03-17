// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::continent::Coord;
use nil_core::infrastructure::storage::OverallStorageCapacity;
use nil_core::military::Military;
use nil_core::player::{Player, PlayerStatus, PublicPlayer};
use nil_core::report::ReportId;
use nil_core::resources::maintenance::Maintenance;
use nil_core::world::config::WorldId;
use nil_payload::player::*;

impl Client {
  pub async fn get_player(&self, req: GetPlayerRequest) -> Result<Player> {
    http::json_put("get-player")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_player_coords(&self, req: GetPlayerCoordsRequest) -> Result<Vec<Coord>> {
    http::json_put("get-player-coords")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_player_maintenance(
    &self,
    req: GetPlayerMaintenanceRequest,
  ) -> Result<Maintenance> {
    http::json_put("get-player-maintenance")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_player_military(&self, req: GetPlayerMilitaryRequest) -> Result<Military> {
    http::json_put("get-player-military")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_player_reports(&self, req: GetPlayerReportsRequest) -> Result<Vec<ReportId>> {
    http::json_put("get-player-reports")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_player_status(&self, req: GetPlayerStatusRequest) -> Result<PlayerStatus> {
    http::json_put("get-player-status")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_player_storage_capacity(
    &self,
    req: GetPlayerStorageCapacityRequest,
  ) -> Result<OverallStorageCapacity> {
    http::json_put("get-player-storage-capacity")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_player_worlds(&self, req: GetPlayerWorldsRequest) -> Result<Vec<WorldId>> {
    http::json_put("get-player-worlds")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_public_player(&self, req: GetPublicPlayerRequest) -> Result<PublicPlayer> {
    http::json_put("get-public-player")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_public_players(
    &self,
    req: GetPublicPlayersRequest,
  ) -> Result<Vec<PublicPlayer>> {
    http::json_put("get-public-players")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn player_exists(&self, req: PlayerExistsRequest) -> Result<bool> {
    http::json_put("player-exists")
      .body(req)
      .server(self.server)
      .circuit_breaker(self.circuit_breaker())
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn set_player_status(&self, req: SetPlayerStatusRequest) -> Result<()> {
    http::post("set-player-status")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn spawn_player(&self, req: SpawnPlayerRequest) -> Result<()> {
    http::post("spawn-player")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .circuit_breaker(self.circuit_breaker())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
