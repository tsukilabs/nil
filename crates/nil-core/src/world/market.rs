// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::market::Market;
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  pub fn market(&self) -> &Market {
    &self.market
  }

  pub(crate) fn market_mut(&mut self) -> &mut Market {
    &mut self.market
  }
}
