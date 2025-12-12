// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::city::City;
use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::npc::bot::BotId;
use crate::with_random_level;
use crate::world::World;

impl World {
  pub(crate) fn spawn_bots(&mut self) -> Result<()> {
    let size = usize::from(self.continent.size());
    let count = size.saturating_mul(2);
    for name in nil_namegen::generate(count) {
      self.spawn_bot(name)?;
    }

    Ok(())
  }

  pub(crate) fn spawn_bot(&mut self, id: impl Into<BotId>) -> Result<BotId> {
    let id: BotId = id.into();
    self.bot_manager.manage(id.clone())?;
    let (coord, field) = self.find_spawn_point()?;

    let infrastructure = Infrastructure::builder()
      .farm(with_random_level!(Farm, 1, 10))
      .iron_mine(with_random_level!(IronMine, 1, 10))
      .prefecture(with_random_level!(Prefecture, 1, 5))
      .quarry(with_random_level!(Quarry, 1, 10))
      .sawmill(with_random_level!(Sawmill, 1, 10))
      .silo(with_random_level!(Silo, 10, 15))
      .warehouse(with_random_level!(Warehouse, 10, 15))
      .build();

    *field = City::builder(coord)
      .name(id.as_ref())
      .owner(id.clone())
      .infrastructure(infrastructure)
      .build()
      .into();

    Ok(id)
  }
}
