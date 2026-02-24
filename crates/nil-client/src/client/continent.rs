// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_core::continent::{ContinentSize, Coord, PublicField};
use nil_payload::continent::*;

impl Client {
  pub async fn get_continent_size(&self, req: GetContinentSizeRequest) -> Result<ContinentSize> {
    http::json_post("get-continent-size")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_public_field(&self, req: GetPublicFieldRequest) -> Result<PublicField> {
    http::json_post("get-public-field")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn get_public_fields(
    &self,
    req: GetPublicFieldsRequest,
  ) -> Result<Vec<(Coord, PublicField)>> {
    http::json_post("get-public-fields")
      .body(req)
      .server(self.server)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
