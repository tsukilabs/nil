// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::world::World;
use std::num::NonZeroU8;

pub fn skip_round(world: &mut World, amount: NonZeroU8) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);

  if !world.round.is_idle() {
    let amount = amount.get();
    for i in 1..=amount {
      world.dangerously_end_round(i == amount)?;
    }
  }

  Ok(())
}
