// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_not_owned_by;
use crate::world::script::WorldUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};

pub(super) fn add_methods<'a, M: UserDataMethods<WorldUserData<'a>>>(methods: &mut M) {
  methods.add_method_mut(
    "toggle_building",
    |lua, this, (coord, id, enabled): (Value, Value, bool)| {
      let coord = lua.from_value(coord)?;
      let id = lua.from_value(id)?;
      bail_not_owned_by!(this, coord);
      this
        .world
        .toggle_building(coord, id, enabled)
        .map_err(Into::into)
    },
  );
}
