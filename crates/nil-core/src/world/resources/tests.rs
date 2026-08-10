// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::player::PlayerId;
use crate::tests::{res, spawn_player};
use crate::world::{World, WorldOptions};
use tap::TryConv;

#[test]
#[cfg_attr(miri, ignore)]
fn withdraw_resources_up_to() -> Result<()> {
  let mut world = WorldOptions::builder("World")
    .build()
    .try_conv::<World>()?;

  let player = PlayerId::from("Player A");
  spawn_player(&mut world, player.as_str())?;

  world.player_mut(&player)?.resources = res(10_000);

  let mut buf = res(8000);

  world.withdraw_resources_up_to(&player, &mut buf)?;
  assert_eq!(world.player(&player)?.resources(), res(2_000));
  assert_eq!(buf, res(8000));

  world.withdraw_resources_up_to(&player, &mut buf)?;
  assert_eq!(world.player(&player)?.resources(), res(0));
  assert_eq!(buf, res(2_000));

  Ok(())
}
