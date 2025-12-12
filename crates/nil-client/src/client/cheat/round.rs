// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_payload::cheat::round::CheatSkipRoundRequest;

impl Client {
  pub async fn cheat_skip_round(&self, req: CheatSkipRoundRequest) -> Result<()> {
    self.http.post("cheat-skip-round", req).await
  }
}
