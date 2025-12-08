// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::infrastructure::storage::OverallStorageCapacity;
use nil_core::military::Military;
use nil_core::player::{Player, PlayerStatus, PublicPlayer};
use nil_core::report::ReportId;
use nil_core::resources::Maintenance;
use nil_payload::player::{
  GetPlayerCoordsRequest,
  GetPlayerRequest,
  GetPlayerStatusRequest,
  GetPublicPlayerRequest,
  PlayerExistsRequest,
  SetPlayerStatusRequest,
  SpawnPlayerRequest,
};

impl Client {
  pub async fn get_player(&self, req: GetPlayerRequest) -> Result<Player> {
    self.http.json_post("get-player", req).await
  }

  pub async fn get_player_coords(&self, req: GetPlayerCoordsRequest) -> Result<Vec<Coord>> {
    self
      .http
      .json_post("get-player-coords", req)
      .await
  }

  pub async fn get_player_maintenance(&self) -> Result<Maintenance> {
    self
      .http
      .json_get("get-player-maintenance")
      .await
  }

  pub async fn get_player_military(&self) -> Result<Military> {
    self
      .http
      .json_get("get-player-military")
      .await
  }

  pub async fn get_player_reports(&self) -> Result<Vec<ReportId>> {
    self
      .http
      .json_get("get-player-reports")
      .await
  }

  pub async fn get_player_status(&self, req: GetPlayerStatusRequest) -> Result<PlayerStatus> {
    self
      .http
      .json_post("get-player-status", req)
      .await
  }

  pub async fn get_players(&self) -> Result<Vec<Player>> {
    self.http.json_get("get-players").await
  }

  pub async fn get_player_storage_capacity(&self) -> Result<OverallStorageCapacity> {
    self
      .http
      .json_get("get-player-storage-capacity")
      .await
  }

  pub async fn get_public_player(&self, req: GetPublicPlayerRequest) -> Result<PublicPlayer> {
    self
      .http
      .json_post("get-public-player", req)
      .await
  }

  pub async fn get_public_players(&self) -> Result<Vec<PublicPlayer>> {
    self
      .http
      .json_get("get-public-players")
      .await
  }

  pub async fn set_player_status(&self, req: SetPlayerStatusRequest) -> Result<()> {
    self
      .http
      .post("set-player-status", req)
      .await
  }

  pub async fn spawn_player(&self, req: SpawnPlayerRequest) -> Result<()> {
    self.http.post("spawn-player", req).await
  }

  pub async fn player_exists(&self, req: PlayerExistsRequest) -> Result<bool> {
    self
      .http
      .json_post("player-exists", req)
      .await
  }
}
