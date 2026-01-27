// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::continent::{ContinentSize, Coord, PublicField};
use nil_payload::continent::*;

impl Client {
  pub async fn get_continent_size(&self, req: GetContinentSizeRequest) -> Result<ContinentSize> {
    self
      .http
      .json_post("get-continent-size", req)
      .await
  }

  pub async fn get_public_field(&self, req: GetPublicFieldRequest) -> Result<PublicField> {
    self
      .http
      .json_post("get-public-field", req)
      .await
  }

  pub async fn get_public_fields(
    &self,
    req: GetPublicFieldsRequest,
  ) -> Result<Vec<(Coord, PublicField)>> {
    self
      .http
      .json_post("get-public-fields", req)
      .await
  }
}
