// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_payload::cheat::military::*;

impl Client {
  pub async fn cheat_spawn_personnel(&self, req: CheatSpawnPersonnelRequest) -> Result<()> {
    self
      .http
      .post("cheat-spawn-personnel", req)
      .await
  }
}
