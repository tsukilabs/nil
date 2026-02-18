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
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::mpsc::{self, UnboundedReceiver};

pub struct Lua {
  inner: mlua::Lua,
  client: Arc<RwLock<Client>>,
  stdout: Stdio,
  stderr: Stdio,
}

impl Lua {
  pub fn new(client: Arc<RwLock<Client>>) -> Result<Self> {
    Self::with_libs(client, StdLib::MATH | StdLib::STRING | StdLib::TABLE)
  }

  pub fn with_client(client: Client) -> Result<Self> {
    Self::new(Arc::new(RwLock::new(client)))
  }

  pub fn with_client_and_libs(client: Client, libs: StdLib) -> Result<Self> {
    Self::with_libs(Arc::new(RwLock::new(client)), libs)
  }

  pub fn with_libs(client: Arc<RwLock<Client>>, libs: StdLib) -> Result<Self> {
    let lua = mlua::Lua::new_with(libs, LuaOptions::default())?;

    let globals = lua.globals();
    let client_data = ClientUserData::new(Arc::clone(&client));
    globals.set("client", lua.create_userdata(client_data)?)?;

    let stdout_rx = set_stdio_pipe(&lua, "print", "println")?;
    let stderr_rx = set_stdio_pipe(&lua, "eprint", "eprintln")?;

    Ok(Self {
      inner: lua,
      client,
      stdout: Stdio::new(stdout_rx),
      stderr: Stdio::new(stderr_rx),
    })
  }

  pub fn client(&self) -> Arc<RwLock<Client>> {
    Arc::clone(&self.client)
  }

  pub fn output(&mut self) -> Output<'_> {
    self.flush();

    Output {
      stdout: &self.stdout.buffer,
      stderr: &self.stderr.buffer,
    }
  }

  pub async fn execute(&mut self, chunk: &str) -> Result<Output<'_>> {
    self.flush();
    self.clear();

    self.inner.load(chunk).exec_async().await?;

    Ok(self.output())
  }

  pub fn flush(&mut self) {
    self.stdout.flush();
    self.stderr.flush();
  }

  fn clear(&mut self) {
    self.stdout.buffer.clear();
    self.stderr.buffer.clear();
  }
}

fn set_stdio_pipe(lua: &mlua::Lua, name: &str, name_ln: &str) -> Result<UnboundedReceiver<String>> {
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

pub struct Output<'a> {
  pub stdout: &'a str,
  pub stderr: &'a str,
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
