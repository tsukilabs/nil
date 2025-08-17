// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::Client;
use crate::error::Result;
use nil_core::continent::Coord;
use nil_core::npc::precursor::{PrecursorId, PublicPrecursor};

impl Client {
  /// GET `/npc/precursor/{id}/coord`
  pub async fn get_precursor_coords(&self, id: PrecursorId) -> Result<Vec<Coord>> {
    self
      .http
      .get_json(&format!("npc/precursor/{id}/coord"))
      .await
  }

  /// GET `/npc/precursor/{id}/public`
  pub async fn get_public_precursor(&self, id: PrecursorId) -> Result<PublicPrecursor> {
    self
      .http
      .get_json(&format!("npc/precursor/{id}/public"))
      .await
  }
}
