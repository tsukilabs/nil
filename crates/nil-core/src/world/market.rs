// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

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

  pub fn send_resources(&mut self, from: &Ruler, to: &Ruler, resources: Resources) -> Result<()> {
    if from == to {
      return Err(Error::ResourcesTransferToSelf(from.clone()));
    }

    let fee = resources * self.market().fee();
    let total = resources + fee;

    let mut ruler_ref = self.ruler_mut(from)?;
    let ruler_resources = ruler_ref.resources_mut();

    match ruler_resources.checked_sub(total) {
      Some(new) => *ruler_resources = new,
      None => return Err(Error::InsufficientResources),
    }

    *self.ruler_mut(to)?.resources_mut() += resources;

    self.emit_ruler(from)?;
    self.emit_ruler(to)?;

    Ok(())
  }
}
