// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use crate::http;
use nil_core::continent::Coord;
use nil_core::npc::precursor::PublicPrecursor;
use nil_payload::npc::precursor::*;

impl Client {
  pub async fn get_precursor_coords(&self, req: GetPrecursorCoordsRequest) -> Result<Vec<Coord>> {
    http::json_post("get-precursor-coords")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_public_precursor(
    &self,
    req: GetPublicPrecursorRequest,
  ) -> Result<PublicPrecursor> {
    http::json_post("get-public-precursor")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_public_precursors(
    &self,
    req: GetPublicPrecursorsRequest,
  ) -> Result<Vec<PublicPrecursor>> {
    http::json_post("get-public-precursors")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
