// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::npc::bot::PublicBot;
use nil_payload::npc::bot::*;

impl Client {
  pub async fn get_bot_coords(&self, req: GetBotCoordsRequest) -> Result<Vec<Coord>> {
    self
      .http
      .json_post("get-bot-coords", req)
      .await
  }

  pub async fn get_public_bot(&self, req: GetPublicBotRequest) -> Result<PublicBot> {
    self
      .http
      .json_post("get-public-bot", req)
      .await
  }
}
