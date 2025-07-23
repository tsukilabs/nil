// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod infrastructure;
mod npc;
mod resources;
mod village;

#[doc(hidden)]
#[macro_export]
macro_rules! bail_cheat_not_allowed {
  ($world:expr) => {
    if !$world.config.allow_cheats {
      use $crate::error::Error;
      return Err(Error::CheatingNotAllowed);
    }
  };
}
