// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::bail_if_cheats_are_not_allowed;
use crate::error::Result;
use crate::market::fee::MarketFee;
use crate::resources::Resources;
use crate::world::World;

pub fn set_market_fee(world: &mut World, fee: MarketFee) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);
  world.market.fee = fee.clamped();
  world.emit_market()?;
  Ok(())
}

pub fn set_market_vault_resources(world: &mut World, resources: Resources) -> Result<()> {
  bail_if_cheats_are_not_allowed!(world);
  world.market.vault.resources = resources;
  world.emit_market()?;
  Ok(())
}
