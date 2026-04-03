// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_favicon_url = "https://nil.dev.br/favicon.png")]
#![feature(iterator_try_collect)]

pub mod client;
pub mod error;
pub mod io;
pub mod script;

use client::ClientUserData;
use error::Result;
use io::{Stdio, StdioMessage, Writer};
use mlua::{LuaOptions, StdLib, Value, Variadic};
use nil_client::Client;
use script::ScriptOutput;
use std::mem;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{self, UnboundedReceiver};

pub struct Lua {
  inner: mlua::Lua,
  stdout: Stdio,
  stderr: Stdio,
  // stdout2: Writer,
}

#[bon::bon]
impl Lua {
  #[builder]
  pub fn new(
    #[builder(start_fn)] client: &Arc<RwLock<Client>>,
    #[builder(default = StdLib::ALL_SAFE)] libs: StdLib,
  ) -> Result<Self> {
    let lua = mlua::Lua::new_with(libs, LuaOptions::default())?;

    let globals = lua.globals();
    let client_data = ClientUserData::new(Arc::clone(client));
    globals.set("client", lua.create_userdata(client_data)?)?;

    let stdout_rx = pipe(&lua, "print", "println")?;
    let stderr_rx = pipe(&lua, "eprint", "eprintln")?;

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
    self.stdout.buffer.sort();
    self.stderr.buffer.sort();

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

fn pipe(lua: &mlua::Lua, name: &str, name_ln: &str) -> Result<UnboundedReceiver<StdioMessage>> {
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

      let _ = tx.send(StdioMessage::new(string));

      Ok(())
    })
  };

  let globals = lua.globals();
  globals.set(name, create_fn(false)?)?;
  globals.set(name_ln, create_fn(true)?)?;

  Ok(rx)
}
