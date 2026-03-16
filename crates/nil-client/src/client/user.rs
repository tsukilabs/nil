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
      .user_agent(&self.user_agent)
      .send()
      .await
  }

  pub async fn user_exists(&self, req: UserExistsRequest) -> Result<bool> {
    http::json_put("user-exists")
      .body(req)
      .server(self.server)
      .retry(&self.retry)
      .user_agent(&self.user_agent)
      .send()
      .await
  }
}
