// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::market::Market;
use crate::resources::Resources;
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  pub fn market(&self) -> &Market {
    &self.market
  }

  pub(crate) fn market_mut(&mut self) -> &mut Market {
    &mut self.market
  }

  pub fn sell_resources(&mut self, ruler: &Ruler, resources: Resources) -> Result<()> {
    self
      .ruler_mut(ruler)?
      .remove_resources(resources)?;

    self
      .market_mut()
      .vault_mut()
      .store(resources);

    self.emit_ruler(ruler)?;
    self.emit_market()?;

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
      .remove_resources(total)?;

    self.add_resources_within_capacity(to.clone(), resources)?;

    self.emit_ruler(from)?;
    self.emit_ruler(to)?;

    self.market_mut().vault_mut().store(fee);
    self.emit_market()?;

    Ok(())
  }
}
