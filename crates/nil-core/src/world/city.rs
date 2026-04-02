// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::city::City;
use crate::continent::ContinentKey;
use crate::error::Result;
use crate::ranking::score::Score;
use crate::ruler::Ruler;
use crate::world::World;

impl World {
  #[inline]
  pub fn city(&self, key: impl ContinentKey) -> Result<&City> {
    self.continent.city(key)
  }

  #[inline]
  pub(crate) fn city_mut(&mut self, key: impl ContinentKey) -> Result<&mut City> {
    self.continent.city_mut(key)
  }

  pub fn count_cities<R>(&self, owner: R) -> u32
  where
    R: Into<Ruler>,
  {
    self
      .continent
      .cities_of(owner)
      .count()
      .try_into()
      .unwrap_or(u32::MAX)
  }

  pub fn get_city_score(&self, key: impl ContinentKey) -> Result<Score> {
    let stats = self.stats.infrastructure.as_ref();
    self.continent.city(key)?.score(stats)
  }

  pub fn rename_city(&mut self, key: impl ContinentKey, name: &str) -> Result<()> {
    let coord = key.into_coord(self.continent.size())?;
    let city = self.continent.city_mut(coord)?;
    name.clone_into(city.name_mut());

    self.emit_public_city_updated(coord);
    self.emit_city_updated(coord);

    Ok(())
  }
}
