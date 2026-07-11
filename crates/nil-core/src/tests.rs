// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::coord::Coord;
use crate::error::Result;
use crate::infrastructure::stats::InfrastructureStats;
use crate::npc::bot::{Bot, BotId};
use crate::player::{PlayerId, PlayerOptions};
use crate::resources::Resources;
use crate::ruler::Ruler;
use crate::world::config::WorldConfig;
use crate::world::{World, WorldOptions};
use std::sync::LazyLock;

pub(crate) static CONFIG: LazyLock<WorldConfig> = LazyLock::new(|| {
  WorldConfig::builder("World")
    .allow_cheats(true)
    .build()
});

pub(crate) static INFRASTRUCTURE_STATS: LazyLock<InfrastructureStats> =
  LazyLock::new(|| InfrastructureStats::new(&CONFIG));

pub(crate) fn get_first_coord<R>(world: &World, ruler: R) -> Coord
where
  R: Into<Ruler>,
{
  world
    .continent()
    .coords_of(ruler)
    .next()
    .unwrap()
}

pub(crate) fn make_world() -> Result<World> {
  WorldOptions::builder("World")
    .allow_cheats(true)
    .build()
    .to_world()
}

pub(crate) fn make_ruler_bot(id: impl Into<BotId>) -> Ruler {
  Ruler::from(&Bot::new(id.into()))
}

pub(crate) fn res(value: u32) -> Resources {
  Resources::splat(value)
}

pub(crate) fn spawn_player(world: &mut World, id: impl Into<PlayerId>) -> Result<()> {
  PlayerOptions::builder(id)
    .build()
    .into_player()
    .spawn(world)
}
