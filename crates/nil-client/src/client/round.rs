// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::round::Round;
use nil_payload::round::SetPlayerReadyRequest;

impl Client {
  /// GET `/round`
  pub async fn get_round(&self) -> Result<Round> {
    self.http.get_json("round").await
  }

  /// POST `/round/ready`
  pub async fn set_player_ready(&self, req: SetPlayerReadyRequest) -> Result<()> {
    self.http.post("round/ready", req).await
  }

  /// GET `/round/start`
  pub async fn start_round(&self) -> Result<()> {
    self.http.get("round/start").await
  }
}
