// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_payload::cheat::round::*;

impl Client {
  pub async fn cheat_skip_round(&self, req: CheatSkipRoundRequest) -> Result<()> {
    http::post("cheat-skip-round")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
