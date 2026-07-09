// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

mod building;
mod storage;

use crate::city::City;
use crate::continent::index::ContinentKey;
use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::world::World;

impl World {
  #[inline]
  pub fn infrastructure(&self, key: impl ContinentKey) -> Result<&Infrastructure> {
    self.city(key).map(City::infrastructure)
  }
}
