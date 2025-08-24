// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::world::script::WorldUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};

pub(super) fn add_methods<'a, M: UserDataMethods<WorldUserData<'a>>>(methods: &mut M) {
  methods.add_method("get_player_maintenance", |lua, this, ()| {
    this
      .world
      .get_player_maintenance(&this.player)
      .map(|maintenance| lua.to_value(&maintenance))?
  });

  methods.add_method("get_player_storage_capacity", |lua, this, ()| {
    this
      .world
      .get_storage_capacity(&this.player)
      .map(|capacity| lua.to_value(&capacity))?
  });

  methods.add_method("has_player", |lua, this, player: Value| {
    let player = lua.from_value(player)?;
    Ok(this.world.has_player(&player))
  });
}
