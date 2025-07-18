// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::world::script::WorldUserData;
use mlua::UserDataMethods;

pub(super) fn add_methods<'a, M: UserDataMethods<WorldUserData<'a>>>(methods: &mut M) {
  methods.add_method_mut("end_turn", |_, this, ()| {
    this
      .world
      .end_turn(&this.player)
      .map_err(Into::into)
  });
}
