// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::round::Round;
use nil_payload::round::*;

impl Client {
  pub async fn get_round(&self, req: GetRoundRequest) -> Result<Round> {
    http::json_post("get-round")
      .body(req)
      .server(self.server)
      .send()
      .await
  }

  pub async fn set_player_ready(&self, req: SetPlayerReadyRequest) -> Result<()> {
    http::post("set-player-ready")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }

  pub async fn start_round(&self, req: StartRoundRequest) -> Result<()> {
    http::post("start-round")
      .body(req)
      .server(self.server)
      .maybe_authorization(self.authorization.as_ref())
      .send()
      .await
  }
}
