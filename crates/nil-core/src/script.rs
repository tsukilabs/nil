// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::player::PlayerId;
use derive_more::{Deref, Display};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const EXTENSION: &str = "lua";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scripting {
  current_id: ScriptId,
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
  pub fn add(&mut self, mut script: Script) -> ScriptId {
    self.current_id = self.current_id.next();
    script.id = self.current_id;
    self.scripts.insert(self.current_id, script);
    self.current_id
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Script {
  #[serde(default)]
  pub id: ScriptId,
  pub name: String,
  pub code: String,
  pub owner: PlayerId,
}

#[derive(
  Clone,
  Copy,
  Debug,
  Default,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Deref,
  Display,
  Deserialize,
  Serialize,
)]
pub struct ScriptId(u32);

impl ScriptId {
  #[inline]
  #[must_use]
  const fn next(self) -> Self {
    Self(self.0.wrapping_add(1))
  }
}

#[must_use]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stdio {
  stdout: Vec<Box<str>>,
}

impl Stdio {
  pub(crate) fn push_stdout(&mut self, value: &str) {
    self.stdout.push(Box::from(value));
  }
}
