// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_payload::cheat::infrastructure::{
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
}
