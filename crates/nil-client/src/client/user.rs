// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use crate::http;
use nil_payload::user::*;

impl Client {
  pub async fn create_user(&self, req: CreateUserRequest) -> Result<()> {
    http::post("create-user")
      .body(req)
      .server(self.server)
      .send()
      .await
  }
}
