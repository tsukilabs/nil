// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::market::Market;
use crate::resources::Resources;
use crate::resources::diff::ResourcesDiff;
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  pub fn market(&self) -> &Market {
    &self.market
  }

  pub(crate) fn market_mut(&mut self) -> &mut Market {
    &mut self.market
  }

  /// Sends resources from one ruler to another, also deducting the current market fee from the sender's resources.
  pub fn send_resources(&mut self, from: &Ruler, to: &Ruler, resources: Resources) -> Result<()> {
    if from == to {
      return Err(Error::ResourceReceiverIsSender(from.clone()));
    }

    let fee = resources * self.market().fee();
    let total = resources + fee;

    let capacity = self.get_storage_capacity(to.clone())?;
    let resources_diff = ResourcesDiff::try_from(resources)?;

    let mut ruler_ref = self.ruler_mut(from)?;
    let ruler_resources = ruler_ref.resources_mut();

    match ruler_resources.checked_sub(total) {
      Some(new) => *ruler_resources = new,
      None => return Err(Error::InsufficientResources),
    }

    self
      .ruler_mut(to)?
      .resources_mut()
      .add_within_capacity(resources_diff, capacity);

    self.emit_ruler(from)?;
    self.emit_ruler(to)?;

    self.market_mut().vault_mut().store(fee);
    self.emit_market()?;

    Ok(())
  }
}
