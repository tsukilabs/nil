// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::infrastructure::storage::OverallStorageCapacity;
use nil_payload::cheat::infrastructure::{
  CheatGetStorageCapacityRequest,
  CheatSetBuildingLevelRequest,
  CheatSetMaxInfrastructureRequest,
};

impl Client {
  /// POST `/cheat/infrastructure`
  pub async fn cheat_set_max_infrastructure(
    &self,
    req: CheatSetMaxInfrastructureRequest,
  ) -> Result<()> {
    self
      .http
      .post("cheat/infrastructure", req)
      .await
  }

  /// POST `/cheat/infrastructure/building`
  pub async fn cheat_set_building_level(&self, req: CheatSetBuildingLevelRequest) -> Result<()> {
    self
      .http
      .post("cheat/infrastructure/building", req)
      .await
  }

  /// POST `/cheat/infrastructure/storage`
  pub async fn cheat_get_storage_capacity(
    &self,
    req: CheatGetStorageCapacityRequest,
  ) -> Result<OverallStorageCapacity> {
    self
      .http
      .post_json("cheat/infrastructure/storage", req)
      .await
  }
}
