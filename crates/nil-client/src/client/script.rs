// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::script::{Script, ScriptId, Stdio};

impl Client {
  /// POST `/script`
  pub async fn get_script(&self, id: ScriptId) -> Result<Script> {
    self.http.post_json("script", id).await
  }

  /// POST `/script/add`
  pub async fn add_scripts(&self, scripts: Vec<Script>) -> Result<Vec<ScriptId>> {
    self
      .http
      .post_json("script/add", scripts)
      .await
  }

  /// POST `/script/all`
  pub async fn get_scripts(&self) -> Result<Vec<Script>> {
    self
      .http
      .post_json("script/all", &self.player)
      .await
  }

  /// POST `/script/chunk`
  pub async fn execute_script_chunk(&self, chunk: &str) -> Result<Stdio> {
    self
      .http
      .post_json("script/chunk", chunk)
      .await
  }

  /// POST `/script/execute`
  pub async fn execute_script(&self, id: ScriptId) -> Result<Stdio> {
    self
      .http
      .post_json("script/execute", id)
      .await
  }

  /// POST `/script/remove`
  pub async fn remove_script(&self, id: ScriptId) -> Result<()> {
    self.http.post("script/remove", id).await
  }

  /// POST `/script/update`
  pub async fn update_script(&self, script: Script) -> Result<()> {
    self.http.post("script/update", script).await
  }
}
