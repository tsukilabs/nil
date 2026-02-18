// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(iterator_try_collect)]

pub mod client;
pub mod error;

use crate::client::ClientUserData;
use crate::error::Result;
use mlua::{LuaOptions, StdLib, Value, Variadic};
use nil_client::Client;
use serde::{Deserialize, Serialize};
use std::mem;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{self, UnboundedReceiver};

pub struct Lua {
  inner: mlua::Lua,
  stdout: Stdio,
  stderr: Stdio,
}

impl Lua {
  pub fn new(client: &Arc<RwLock<Client>>) -> Result<Self> {
    Self::with_libs(client, StdLib::MATH | StdLib::STRING | StdLib::TABLE)
  }

  pub fn with_libs(client: &Arc<RwLock<Client>>, libs: StdLib) -> Result<Self> {
    let lua = mlua::Lua::new_with(libs, LuaOptions::default())?;

    let globals = lua.globals();
    let client_data = ClientUserData::new(Arc::clone(client));
    globals.set("client", lua.create_userdata(client_data)?)?;

    let stdout_rx = pipe_stdio(&lua, "print", "println")?;
    let stderr_rx = pipe_stdio(&lua, "eprint", "eprintln")?;

    Ok(Self {
      inner: lua,
      stdout: Stdio::new(stdout_rx),
      stderr: Stdio::new(stderr_rx),
    })
  }

  pub async fn execute(&mut self, chunk: &str) -> Result<ScriptOutput> {
    self.flush();
    self.clear();

    self.inner.load(chunk).exec_async().await?;

    Ok(self.output())
  }

  fn output(&mut self) -> ScriptOutput {
    self.flush();

    ScriptOutput {
      stdout: mem::take(&mut self.stdout.buffer),
      stderr: mem::take(&mut self.stderr.buffer),
    }
  }

  fn flush(&mut self) {
    self.stdout.flush();
    self.stderr.flush();
  }

  fn clear(&mut self) {
    self.stdout.buffer.clear();
    self.stderr.buffer.clear();
  }
}

fn pipe_stdio(lua: &mlua::Lua, name: &str, name_ln: &str) -> Result<UnboundedReceiver<String>> {
  let (tx, rx) = mpsc::unbounded_channel();
  let create_fn = |line_break: bool| {
    let tx = tx.clone();
    lua.create_function(move |_, values: Variadic<Value>| {
      let mut string = values
        .into_iter()
        .map(|it| it.to_string())
        .try_collect::<String>()?;

      if line_break {
        string.push('\n');
      }

      let _ = tx.send(string);

      Ok(())
    })
  };

  let globals = lua.globals();
  globals.set(name, create_fn(false)?)?;
  globals.set(name_ln, create_fn(true)?)?;

  Ok(rx)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptOutput {
  pub stdout: String,
  pub stderr: String,
}

pub struct Stdio {
  buffer: String,
  receiver: UnboundedReceiver<String>,
}

impl Stdio {
  fn new(receiver: UnboundedReceiver<String>) -> Self {
    Self { buffer: String::new(), receiver }
  }

  fn flush(&mut self) {
    while let Ok(value) = self.receiver.try_recv() {
      self.buffer.push_str(&value);
    }
  }
}
