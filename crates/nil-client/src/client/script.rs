// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Client;
use crate::error::Result;
use nil_core::script::{AddScriptRequest, Script, ScriptId, Stdout};

impl Client {
  /// GET `/script`
  pub async fn get_scripts(&self) -> Result<Vec<Script>> {
    self.http.get_json("script").await
  }

  /// POST `/script`
  pub async fn add_scripts(&self, scripts: Vec<AddScriptRequest>) -> Result<Vec<ScriptId>> {
    self.http.post_json("script", scripts).await
  }

  /// POST `/script/chunk`
  pub async fn execute_script_chunk(&self, chunk: &str) -> Result<Stdout> {
    self
      .http
      .post_json("script/chunk", chunk)
      .await
  }

  /// POST `/script/update`
  pub async fn update_script(&self, script: Script) -> Result<()> {
    self.http.post("script/update", script).await
  }

  /// GET `/script/{id}`
  pub async fn get_script(&self, id: ScriptId) -> Result<Script> {
    self
      .http
      .get_json(&format!("script/{id}"))
      .await
  }

  /// GET `/script/{id}/execute`
  pub async fn execute_script(&self, id: ScriptId) -> Result<Stdout> {
    self
      .http
      .get_json(&format!("script/{id}/execute"))
      .await
  }

  /// GET `/script/{id}/remove`
  pub async fn remove_script(&self, id: ScriptId) -> Result<()> {
    self
      .http
      .get(&format!("script/{id}/remove"))
      .await
  }
}
