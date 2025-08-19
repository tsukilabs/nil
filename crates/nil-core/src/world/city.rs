// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::World;
use crate::city::CityOwner;
use crate::continent::ContinentKey;
use crate::error::Result;
use crate::ranking::Score;

impl World {
  pub fn count_cities<O>(&self, owner: O) -> u32
  where
    O: Into<CityOwner>,
  {
    let owner: CityOwner = owner.into();
    self
      .continent
      .cities_by(|city| city.owner() == &owner)
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
