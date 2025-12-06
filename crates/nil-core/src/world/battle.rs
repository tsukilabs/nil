// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::battle::{Battle, BattleResult};
use crate::error::Result;
use crate::infrastructure::prelude::BuildingLevel;
use crate::military::squad::Squad;
use crate::world::World;

impl World {
  pub fn simulate_battle(
    &self,
    attacker: &[Squad],
    defender: &[Squad],
    wall: BuildingLevel,
  ) -> Result<BattleResult> {
    let wall_stats = (wall > BuildingLevel::ZERO)
      .then(|| self.stats.infrastructure.wall().get(wall))
      .transpose()?;

    let result = Battle::builder()
      .attacker(attacker)
      .defender(defender)
      .maybe_wall(wall_stats)
      .build()
      .result();

    Ok(result)
  }
}
