// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::player::PlayerId;
use crate::tests::{res, spawn_player};
use crate::world::WorldOptions;

#[test]
#[cfg_attr(miri, ignore)]
fn take_resources_of() -> Result<()> {
  let mut world = WorldOptions::builder("World")
    .build()
    .to_world()?;

  let player = PlayerId::from("Player A");
  spawn_player(&mut world, player.as_str())?;

  *world.player_mut(&player)?.resources_mut() = res(10_000);

  let mut buf = res(8000);

  world.take_resources_of(&player, &mut buf)?;
  assert_eq!(world.player(&player)?.resources(), &res(2_000));
  assert_eq!(buf, res(8000));

  world.take_resources_of(&player, &mut buf)?;
  assert_eq!(world.player(&player)?.resources(), &res(0));
  assert_eq!(buf, res(2_000));

  Ok(())
}
