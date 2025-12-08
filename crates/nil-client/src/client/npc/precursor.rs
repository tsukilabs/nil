// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::npc::precursor::PublicPrecursor;
use nil_payload::npc::precursor::{GetPrecursorCoordsRequest, GetPublicPrecursorRequest};

impl Client {
  pub async fn get_precursor_coords(&self, req: GetPrecursorCoordsRequest) -> Result<Vec<Coord>> {
    self
      .http
      .json_post("get-precursor-coords", req)
      .await
  }

  pub async fn get_public_precursor(
    &self,
    req: GetPublicPrecursorRequest,
  ) -> Result<PublicPrecursor> {
    self
      .http
      .json_post("get-public-precursor", req)
      .await
  }
}
