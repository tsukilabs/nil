// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod city;
mod infrastructure;
mod military;
mod npc;
mod resources;

#[doc(hidden)]
#[macro_export]
macro_rules! bail_cheat_not_allowed {
  ($world:expr) => {
    if !$world.config.are_cheats_allowed() {
      use $crate::error::Error;
      return Err(Error::CheatingNotAllowed);
    }
  };
}
