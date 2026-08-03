// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::market::fee::MarketFee;
use crate::world::World;

pub fn set_market_fee(world: &mut World, fee: MarketFee) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);
  world.market_mut().set_fee(fee);
  world.emit_market()?;
  Ok(())
}
