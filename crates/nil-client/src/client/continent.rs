// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::continent::{ContinentSize, Coord, PublicField};
use nil_payload::continent::{GetPublicFieldRequest, GetPublicFieldsRequest};

impl Client {
  pub async fn get_continent_size(&self) -> Result<ContinentSize> {
    self
      .http
      .json_get("get-continent-size")
      .await
  }

  pub async fn get_field(&self, req: GetPublicFieldRequest) -> Result<PublicField> {
    self.http.json_post("get-field", req).await
  }

  pub async fn get_fields(&self, req: GetPublicFieldsRequest) -> Result<Vec<(Coord, PublicField)>> {
    self.http.json_post("get-fields", req).await
  }
}
