// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::tests::{make_world, spawn_player};
use std::assert_matches;

#[test]
#[cfg_attr(miri, ignore)]
fn cannot_spawn_twice() -> Result<()> {
  let mut world = make_world()?;

  let player_a = PlayerId::from("Player A");
  spawn_player(&mut world, player_a.as_str())?;

  assert!(world.has_player(&player_a));
  assert_matches!(
    spawn_player(&mut world, player_a.as_str()),
    Err(Error::PlayerAlreadySpawned(_))
  );

  Ok(())
}
