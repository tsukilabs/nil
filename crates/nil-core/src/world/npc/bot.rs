// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::city::City;
use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::infrastructure::storage::OverallStorageCapacity;
use crate::npc::bot::{BotId, BotName};
use crate::with_random_level;
use crate::world::World;

impl World {
  pub(crate) fn get_bot_storage_capacity(&self, bot: BotId) -> Result<OverallStorageCapacity> {
    let cities = self.continent.bot_cities_by(|id| id == bot);
    self.get_storage_capacity(cities)
  }

  pub(crate) fn spawn_bots(&mut self) -> Result<()> {
    let size = usize::from(self.continent.size());
    let count = size.saturating_mul(2);
    for name in nil_namegen::generate(count) {
      self.spawn_bot(name)?;
    }

    Ok(())
  }

  pub(crate) fn spawn_bot(&mut self, name: impl Into<BotName>) -> Result<BotId> {
    let name: BotName = name.into();
    let id = self.bot_manager.spawn(name.clone());
    let (coord, field) = self.find_spawn_point()?;

    let infrastructure = Infrastructure::builder()
      .farm(with_random_level!(Farm, 5, 10))
      .iron_mine(with_random_level!(IronMine, 1, 10))
      .prefecture(with_random_level!(Prefecture, 1, 5))
      .quarry(with_random_level!(Quarry, 1, 10))
      .sawmill(with_random_level!(Sawmill, 1, 10))
      .silo(with_random_level!(Silo, 5, 10))
      .warehouse(with_random_level!(Warehouse, 1, 10))
      .build();

    *field = City::builder(coord)
      .name(name)
      .owner(id)
      .infrastructure(infrastructure)
      .build()
      .into();

    Ok(id)
  }
}
