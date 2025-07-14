// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod cheat;
mod global;
mod player;
mod village;

use super::World;
use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::script::ScriptId;
use mlua::{Lua, LuaOptions, StdLib, UserData, UserDataMethods};
use std::sync::LazyLock;

static LUA: LazyLock<Lua> = LazyLock::new(|| {
  Lua::new_with(StdLib::MATH | StdLib::STRING, LuaOptions::default())
    .expect("Failed to create the Lua state")
});

impl World {
  pub fn execute_script(&mut self, id: ScriptId) -> Result<()> {
    let script = self.scripting.get(id).cloned()?;
    self.execute_chunk(script.owner, &script.code)
  }

  fn execute_chunk(&mut self, player: PlayerId, chunk: &str) -> Result<()> {
    WorldUserData::new(self, player).execute(chunk)
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

  fn execute(self, chunk: &str) -> Result<()> {
    let result = LUA.scope(|scope| {
      let globals = LUA.create_table()?;
      let world = scope.create_userdata(self)?;
      globals.set("world", world)?;

      LUA.set_globals(globals)?;

      LUA.load(chunk).exec()
    });

    if let Err(err) = &result
      && let mlua::Error::CallbackError { cause, .. } = err
      && let Some(err) = cause.downcast_ref::<Error>()
    {
      Err(err.clone())
    } else {
      result.map_err(Into::into)
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
      .village($coord)?
      .is_owned_by_player_and(|id| id == &$this.player)
    {
      return Err($crate::error::Error::Forbidden.into());
    }
  };
}

impl UserData for WorldUserData<'_> {
  fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
    cheat::add_methods(methods);
    player::add_methods(methods);
    village::add_methods(methods);
  }
}
