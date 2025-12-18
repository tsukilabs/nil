// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::round::Round;
use nil_payload::round::*;

impl Client {
  pub async fn get_round(&self, req: GetRoundRequest) -> Result<Round> {
    self.http.json_post("get-round", req).await
  }

  pub async fn set_player_ready(&self, req: SetPlayerReadyRequest) -> Result<()> {
    self.http.post("set-player-ready", req).await
  }

  pub async fn start_round(&self, req: StartRoundRequest) -> Result<()> {
    self.http.post("start-round", req).await
  }
}
