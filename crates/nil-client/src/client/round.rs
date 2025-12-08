// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::round::Round;
use nil_payload::round::SetPlayerReadyRequest;

impl Client {
  pub async fn get_round(&self) -> Result<Round> {
    self.http.json_get("get-round").await
  }

  pub async fn set_player_ready(&self, req: SetPlayerReadyRequest) -> Result<()> {
    self.http.post("set-player-ready", req).await
  }

  pub async fn start_round(&self) -> Result<()> {
    self.http.get("start-round").await
  }
}
