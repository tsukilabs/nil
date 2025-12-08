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
  pub async fn cheat_get_storage_capacity(
    &self,
    req: CheatGetStorageCapacityRequest,
  ) -> Result<OverallStorageCapacity> {
    self
      .http
      .json_post("cheat-get-storage-capacity", req)
      .await
  }

  pub async fn cheat_set_building_level(&self, req: CheatSetBuildingLevelRequest) -> Result<()> {
    self
      .http
      .post("cheat-set-building-level", req)
      .await
  }

  pub async fn cheat_set_max_infrastructure(
    &self,
    req: CheatSetMaxInfrastructureRequest,
  ) -> Result<()> {
    self
      .http
      .post("cheat-set-max-infrastructure", req)
      .await
  }
}
