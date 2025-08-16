// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::player::PlayerId;
use bon::Builder;
use derive_more::{Deref, Display};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

pub const EXTENSION: &str = "lua";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scripting {
  scripts: HashMap<ScriptId, Script>,
}

impl Scripting {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  #[inline]
  pub fn get(&self, id: ScriptId) -> Result<&Script> {
    self
      .scripts
      .get(&id)
      .ok_or(Error::ScriptNotFound(id))
  }

  fn get_mut(&mut self, id: ScriptId) -> Result<&mut Script> {
    self
      .scripts
      .get_mut(&id)
      .ok_or(Error::ScriptNotFound(id))
  }

  pub fn get_owned_by(&self, owner: &PlayerId) -> Vec<Script> {
    self
      .scripts
      .values()
      .filter(|it| it.owner.eq(owner))
      .sorted_unstable_by_key(|it| &it.name)
      .cloned()
      .collect()
  }

  /// Adds a new script.
  pub fn add(&mut self, request: AddScriptRequest) -> ScriptId {
    let (id, script) = request.into_script();
    self.scripts.insert(id, script);
    id
  }

  /// Updates an existing script.
  #[inline]
  pub fn update(&mut self, script: Script) -> Result<()> {
    let current = self.get_mut(script.id)?;
    *current = script;
    Ok(())
  }

  /// Removes a script.
  #[inline]
  pub fn remove(&mut self, id: ScriptId) {
    self.scripts.remove(&id);
  }
}

#[derive(Builder, Clone, Debug, Deserialize, Serialize)]
#[builder(builder_type(vis = ""))]
#[serde(rename_all = "camelCase")]
pub struct Script {
  #[builder(start_fn)]
  id: ScriptId,

  #[builder(default = "Script", into)]
  name: String,

  #[builder(default, into)]
  code: String,

  owner: PlayerId,
}

impl Script {
  #[inline]
  pub fn id(&self) -> ScriptId {
    self.id
  }

  #[inline]
  pub fn name(&self) -> &str {
    &self.name
  }

  #[inline]
  pub fn code(&self) -> &str {
    &self.code
  }

  #[inline]
  pub fn owner(&self) -> PlayerId {
    self.owner.clone()
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display, Deserialize, Serialize)]
pub struct ScriptId(Uuid);

impl ScriptId {
  #[must_use]
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Default for ScriptId {
  fn default() -> Self {
    Self::new()
  }
}

#[must_use]
#[derive(Clone, Debug, Default, Deref, Deserialize, Serialize)]
pub struct Stdout(Vec<Arc<str>>);

impl Stdout {
  pub(crate) fn push(&mut self, value: &str) {
    self.0.push(Arc::from(value));
  }
}

impl fmt::Display for Stdout {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0.join("\n"))
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddScriptRequest {
  pub name: Option<String>,
  pub code: Option<String>,
  pub owner: PlayerId,
}

impl AddScriptRequest {
  fn into_script(self) -> (ScriptId, Script) {
    let id = ScriptId::new();
    let script = Script::builder(id)
      .maybe_name(self.name)
      .maybe_code(self.code)
      .owner(self.owner)
      .build();

    (id, script)
  }
}
