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
use nil_payload::player::*;

impl Client {
  pub async fn get_player(&self, req: GetPlayerRequest) -> Result<Player> {
    http::json_post("get-player")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_player_coords(&self, req: GetPlayerCoordsRequest) -> Result<Vec<Coord>> {
    http::json_post("get-player-coords")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_player_maintenance(
    &self,
    req: GetPlayerMaintenanceRequest,
  ) -> Result<Maintenance> {
    http::json_post("get-player-maintenance")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_player_military(&self, req: GetPlayerMilitaryRequest) -> Result<Military> {
    http::json_post("get-player-military")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_player_reports(&self, req: GetPlayerReportsRequest) -> Result<Vec<ReportId>> {
    http::json_post("get-player-reports")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_player_status(&self, req: GetPlayerStatusRequest) -> Result<PlayerStatus> {
    http::json_post("get-player-status")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn get_players(&self, req: GetPlayersRequest) -> Result<Vec<Player>> {
    http::json_post("get-players")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_player_storage_capacity(
    &self,
    req: GetPlayerStorageCapacityRequest,
  ) -> Result<OverallStorageCapacity> {
    http::json_post("get-player-storage-capacity")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn get_public_player(&self, req: GetPublicPlayerRequest) -> Result<PublicPlayer> {
    http::json_post("get-public-player")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn get_public_players(
    &self,
    req: GetPublicPlayersRequest,
  ) -> Result<Vec<PublicPlayer>> {
    http::json_post("get-public-players")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn set_player_status(&self, req: SetPlayerStatusRequest) -> Result<()> {
    http::post("set-player-status")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn spawn_player(&self, req: SpawnPlayerRequest) -> Result<()> {
    http::post("spawn-player")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn player_exists(&self, req: PlayerExistsRequest) -> Result<bool> {
    http::json_post("player-exists")
      .body(req)
      .server(self.server)
      .send()
      .await
  }
}
