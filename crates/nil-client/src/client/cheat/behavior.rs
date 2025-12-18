// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::behavior::build::BuildStep;
use nil_payload::cheat::behavior::*;

impl Client {
  pub async fn cheat_get_build_steps(
    &self,
    req: CheatGetBuildStepsRequest,
  ) -> Result<Vec<BuildStep>> {
    self
      .http
      .json_post("cheat-get-build-steps", req)
      .await
  }
}
