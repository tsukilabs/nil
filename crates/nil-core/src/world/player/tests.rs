// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::player::{PlayerId, PlayerOptions};
use crate::world::{World, WorldOptions};
use std::assert_matches;

#[test]
fn cannot_spawn_twice() -> Result<()> {
  let mut world = make_world()?;

  let player_a = PlayerId::from("Player A");
  spawn(&mut world, &player_a)?;

  assert!(world.has_player(&player_a));
  assert_matches!(
    spawn(&mut world, &player_a),
    Err(Error::PlayerAlreadySpawned(_))
  );

  Ok(())
}

fn make_world() -> Result<World> {
  WorldOptions::builder("World")
    .build()
    .to_world()
}

fn spawn(world: &mut World, id: &PlayerId) -> Result<()> {
  PlayerOptions::builder(id.clone())
    .build()
    .into_player()
    .spawn(world)
}
