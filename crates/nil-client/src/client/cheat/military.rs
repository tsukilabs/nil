// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::military::army::{Army, ArmyPersonnel};
use nil_payload::cheat::military::*;

impl Client {
  pub async fn cheat_get_idle_armies_at(
    &self,
    req: CheatGetIdleArmiesAtRequest,
  ) -> Result<Vec<Army>> {
    http::json_post("cheat-get-idle-armies-at")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_get_idle_personnel_at(
    &self,
    req: CheatGetIdlePersonnelAtRequest,
  ) -> Result<ArmyPersonnel> {
    http::json_post("cheat-get-idle-personnel-at")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn cheat_spawn_personnel(&self, req: CheatSpawnPersonnelRequest) -> Result<()> {
    http::post("cheat-spawn-personnel")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
