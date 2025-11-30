// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::continent::{Coord, PublicField};
use nil_payload::continent::{GetPublicFieldRequest, GetPublicFieldsRequest};

impl Client {
  /// POST `/continent/field`
  pub async fn get_field(&self, req: GetPublicFieldRequest) -> Result<PublicField> {
    self
      .http
      .post_json("continent/field", req)
      .await
  }

  /// POST `/continent/fields`
  pub async fn get_fields(&self, req: GetPublicFieldsRequest) -> Result<Vec<(Coord, PublicField)>> {
    self
      .http
      .post_json("continent/fields", req)
      .await
  }

  /// GET `/continent/size`
  pub async fn get_continent_size(&self) -> Result<usize> {
    self.http.get_json("continent/size").await
  }
}
