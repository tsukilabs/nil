// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::player::{PlayerId, PlayerOptions};
use crate::resources::Resources;
use crate::world::{World, WorldOptions};

#[test]
fn transpose_resources() -> Result<()> {
  let mut world = WorldOptions::builder("World")
    .build()
    .to_world()?;

  let player_a = PlayerId::from("Player A");
  let player_b = PlayerId::from("Player B");

  spawn(&mut world, &player_a)?;
  spawn(&mut world, &player_b)?;

  *world.player_mut(&player_a)?.resources_mut() = res(10_000);
  *world.player_mut(&player_b)?.resources_mut() = res(5_000);

  world.transpose_resources(&player_a, &player_b, res(8_000))?;

  assert_eq!(world.player(&player_a)?.resources(), &res(2_000));
  assert_eq!(world.player(&player_b)?.resources(), &res(13_000));

  world.transpose_resources(&player_a, &player_b, res(8_000))?;

  assert_eq!(world.player(&player_a)?.resources(), &res(0));
  assert_eq!(world.player(&player_b)?.resources(), &res(15_000));

  Ok(())
}

fn spawn(world: &mut World, id: &PlayerId) -> Result<()> {
  PlayerOptions::builder(id.clone())
    .build()
    .into_player()
    .spawn(world)
}

fn res(value: u32) -> Resources {
  Resources::splat(value)
}
