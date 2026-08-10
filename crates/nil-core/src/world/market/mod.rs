// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::market::Market;
use crate::market::MarketOperation::{Buy, Sell};
use crate::resources::Resources;
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  pub fn market(&self) -> &Market {
    &self.market
  }

  /// Buys resources from the market.
  ///
  /// The total gold cost is calculated as `resources + (resources * market_fee)`.
  #[inline]
  pub fn buy_resources(&mut self, ruler: &Ruler, resources: Resources) -> Result<()> {
    self.buy_resources_with_emit(ruler, resources, true)
  }

  pub(crate) fn buy_resources_with_emit(
    &mut self,
    ruler: &Ruler,
    resources: Resources,
    emit: bool,
  ) -> Result<()> {
    let gold = self.market.price_of(Buy, resources);
    self.ruler_mut(ruler)?.withdraw_gold(gold)?;

    self.market.vault.withdraw(resources)?;

    self.add_resources_within_capacity(ruler.clone(), resources)?;

    if emit {
      self.emit_ruler(ruler)?;
      self.emit_market()?;
    }

    Ok(())
  }

  /// Sells resources to the market.
  #[inline]
  pub fn sell_resources(&mut self, ruler: &Ruler, resources: Resources) -> Result<()> {
    self.sell_resources_with_emit(ruler, resources, true)
  }

  pub(crate) fn sell_resources_with_emit(
    &mut self,
    ruler: &Ruler,
    resources: Resources,
    emit: bool,
  ) -> Result<()> {
    self
      .ruler_mut(ruler)?
      .withdraw_resources(resources)?;

    self.market.vault.resources += resources;

    let gold = self.market.price_of(Sell, resources);
    *self.ruler_mut(ruler)?.gold_mut() += gold;

    if emit {
      self.emit_ruler(ruler)?;
      self.emit_market()?;
    }

    Ok(())
  }

  /// Sends resources from one ruler to another, also deducting the current market fee from the sender's resources.
  pub fn send_resources(&mut self, from: &Ruler, to: &Ruler, resources: Resources) -> Result<()> {
    if from == to {
      return Err(Error::ResourceReceiverIsSender(from.clone()));
    }

    let fee = resources * self.market().fee();
    let total = resources + fee;

    self
      .ruler_mut(from)?
      .withdraw_resources(total)?;

    self.add_resources_within_capacity(to.clone(), resources)?;

    self.emit_ruler(from)?;
    self.emit_ruler(to)?;

    self.market.vault.resources += fee;
    self.emit_market()?;

    Ok(())
  }
}
