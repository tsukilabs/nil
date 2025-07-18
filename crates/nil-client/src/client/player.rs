// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::player::{Player, PlayerId, PlayerOptions, PlayerStatus, PlayerStorageCapacity};
use nil_core::resource::Maintenance;

impl Client {
  /// GET `/player`
  pub async fn get_players(&self) -> Result<Vec<Player>> {
    self.http.get_json("player").await
  }

  /// POST `/player`
  pub async fn get_player(&self, id: PlayerId) -> Result<Player> {
    self.http.post_json("player", id).await
  }

  /// GET `/player/capacity`
  pub async fn get_player_storage_capacity(&self) -> Result<PlayerStorageCapacity> {
    self.http.get_json("player/capacity").await
  }

  /// POST `/player/coord`
  pub async fn get_player_coords(&self, id: PlayerId) -> Result<Vec<Coord>> {
    self.http.post_json("player/coord", id).await
  }

  /// POST `/player/exists`
  pub async fn player_exists(&self, id: PlayerId) -> Result<bool> {
    self
      .http
      .post_json("player/exists", id)
      .await
  }

  /// GET `/player/maintenance`
  pub async fn get_player_maintenance(&self) -> Result<Maintenance> {
    self
      .http
      .get_json("player/maintenance")
      .await
  }

  /// POST `/player/set-status`
  pub async fn set_player_status(&self, status: PlayerStatus) -> Result<()> {
    self
      .http
      .post("player/set-status", status)
      .await
  }

  /// POST `/player/spawn`
  pub async fn spawn_player(&self, options: PlayerOptions) -> Result<()> {
    self.http.post("player/spawn", options).await
  }

  /// POST `/player/status`
  pub async fn get_player_status(&self, id: PlayerId) -> Result<PlayerStatus> {
    self
      .http
      .post_json("player/status", id)
      .await
  }
}
