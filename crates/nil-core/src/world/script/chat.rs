// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::world::script::WorldUserData;
use mlua::UserDataMethods;

pub(super) fn add_methods<'a, M: UserDataMethods<WorldUserData<'a>>>(methods: &mut M) {
  methods.add_method_mut("send_chat_message", |_, this, message: String| {
    this
      .world
      .push_chat_message(this.player.clone(), &message);

    Ok(())
  });
}
