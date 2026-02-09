// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::city::City;
use crate::error::Result;
use crate::infrastructure::Infrastructure;
use crate::npc::bot::BotId;
use crate::with_random_level;
use crate::world::World;
use num_traits::ToPrimitive;
use tap::Conv;

impl World {
  pub(crate) fn spawn_bots(&mut self) -> Result<()> {
    let size = self.continent.size();
    let density = self.config().bot_density();
    let count = (f64::from(size) * f64::from(density))
      .ceil()
      .to_usize()
      .unwrap_or_else(|| usize::from(size).saturating_mul(2));

    let advanced_start_ratio = self
      .config
      .bot_advanced_start_ratio()
      .conv::<f64>();

    for name in nil_namegen::generate(count) {
      let infrastructure = if rand::random::<f64>() > advanced_start_ratio {
        Infrastructure::default()
      } else {
        Infrastructure::builder()
          .farm(with_random_level!(Farm, 1, 10))
          .iron_mine(with_random_level!(IronMine, 1, 10))
          .prefecture(with_random_level!(Prefecture, 1, 5))
          .quarry(with_random_level!(Quarry, 1, 10))
          .sawmill(with_random_level!(Sawmill, 1, 10))
          .silo(with_random_level!(Silo, 10, 15))
          .warehouse(with_random_level!(Warehouse, 10, 15))
          .build()
      };

      self.spawn_bot(name, infrastructure)?;
    }

    Ok(())
  }

  pub(crate) fn spawn_bot(
    &mut self,
    id: impl Into<BotId>,
    infrastructure: Infrastructure,
  ) -> Result<BotId> {
    let id: BotId = id.into();
    self.bot_manager.manage(id.clone())?;

    let (coord, field) = self.find_spawn_point()?;

    *field = City::builder(coord)
      .name(id.as_ref())
      .owner(id.clone())
      .infrastructure(infrastructure)
      .build()
      .into();

    Ok(id)
  }
}
