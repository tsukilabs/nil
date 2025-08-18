// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod chat;
mod cheat;
mod city;
mod infrastructure;
mod player;
mod round;

use super::World;
use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::script::{ScriptId, Stdout};
use mlua::{Lua, LuaOptions, StdLib, UserData, UserDataMethods, Value, Variadic};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::LazyLock;

static LUA: LazyLock<Lua> = LazyLock::new(|| {
  Lua::new_with(StdLib::MATH | StdLib::STRING, LuaOptions::default())
    .expect("Failed to create the Lua state")
});

impl World {
  pub fn execute_script(&mut self, id: ScriptId) -> Result<Stdout> {
    let script = self.scripting.get(id).cloned()?;
    self.execute_script_chunk(script.owner(), script.code())
  }

  pub fn execute_script_chunk(&mut self, player: PlayerId, chunk: &str) -> Result<Stdout> {
    let stdout = WorldUserData::new(self, player.clone()).execute(chunk)?;

    if stdout
      .iter()
      .any(|line| !line.trim().is_empty())
    {
      self.push_stdout_message(player, &stdout);
    }

    Ok(stdout)
  }
}

struct WorldUserData<'a> {
  world: &'a mut World,
  player: PlayerId,
}

impl<'a> WorldUserData<'a> {
  fn new(world: &'a mut World, player: PlayerId) -> Self {
    Self { world, player }
  }

  fn execute(self, chunk: &str) -> Result<Stdout> {
    let stdout = Rc::new(RefCell::new(Stdout::default()));
    let result = LUA.scope(|scope| {
      let globals = LUA.create_table()?;
      let world = scope.create_userdata(self)?;
      let print = scope.create_function_mut({
        let stdout = Rc::downgrade(&stdout);
        move |_, values: Variadic<Value>| {
          if let Some(stdio) = stdout.upgrade() {
            for value in values {
              let value = value.to_string()?;
              if let Ok(mut stdio) = stdio.try_borrow_mut() {
                stdio.push(&value);
              }

              #[cfg(debug_assertions)]
              tracing::info!(print = %value);
            }
          }

          Ok(())
        }
      })?;

      globals.set("world", world)?;
      globals.set("print", print)?;

      LUA.set_globals(globals)?;
      LUA.load(chunk).exec()?;

      Ok(())
    });

    if let Err(err) = &result
      && let mlua::Error::CallbackError { cause, .. } = err
      && let Some(err) = cause.downcast_ref::<Error>()
    {
      Err(err.clone())
    } else {
      result?;
      Ok(Rc::unwrap_or_clone(stdout).into_inner())
    }
  }
}

#[doc(hidden)]
#[macro_export]
macro_rules! bail_not_owned_by {
  ($this:expr, $coord:expr) => {
    if !$this
      .world
      .continent
      .city($coord)?
      .is_owned_by_player_and(|id| id == &$this.player)
    {
      return Err($crate::error::Error::Forbidden.into());
    }
  };
}

impl UserData for WorldUserData<'_> {
  fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
    chat::add_methods(methods);
    cheat::add_methods(methods);
    city::add_methods(methods);
    infrastructure::add_methods(methods);
    player::add_methods(methods);
    round::add_methods(methods);
  }
}
