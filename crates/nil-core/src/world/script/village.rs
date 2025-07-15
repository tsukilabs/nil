// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_not_owned_by;
use crate::world::script::WorldUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};

pub(super) fn add_methods<'a, M: UserDataMethods<WorldUserData<'a>>>(methods: &mut M) {
  methods.add_method_mut(
    "rename_village",
    |lua, this, (coord, name): (Value, String)| {
      let coord = lua.from_value(coord)?;
      bail_not_owned_by!(this, coord);
      this
        .world
        .rename_village(coord, &name)
        .map_err(Into::into)
    },
  );
}
