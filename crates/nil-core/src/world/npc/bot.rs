// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::village::Village;
use crate::with_random_level;
use crate::world::World;

impl World {
  pub(crate) fn spawn_bot(&mut self) -> Result<()> {
    let (id, name) = self.bot_manager.spawn();
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

    *field = Village::builder(coord)
      .name(name)
      .owner(id)
      .infrastructure(infrastructure)
      .build()
      .into();

    Ok(())
  }
}
