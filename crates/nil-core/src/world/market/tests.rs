// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::player::PlayerId;
use crate::resources::Resources;
use crate::ruler::Ruler;
use crate::tests::{make_world, spawn_player};
use std::assert_matches;

#[test]
fn send_resources() -> Result<()> {
  let mut world = make_world()?;
  let player_a = PlayerId::from("Player A");
  spawn_player(&mut world, player_a.as_str())?;

  let player_b = PlayerId::from("Player B");
  spawn_player(&mut world, player_b.as_str())?;

  let ruler_a = Ruler::from(player_a);
  let ruler_b = Ruler::from(player_b);
  let initial_resources_a = Resources::splat(5000);
  let initial_resources_b = Resources::splat(1000);

  world
    .ruler_mut(&ruler_a)?
    .resources_mut()
    .set(initial_resources_a);

  world
    .ruler_mut(&ruler_b)?
    .resources_mut()
    .set(initial_resources_b);

  let resources_to_send = Resources::splat(1000);
  world.send_resources(&ruler_a, &ruler_b, resources_to_send)?;

  let remaining_resources_a = world.ruler(&ruler_a)?.resources();
  let fee = resources_to_send * world.market().fee();
  assert_eq!(
    remaining_resources_a,
    initial_resources_a
      .checked_sub(resources_to_send + fee)
      .ok_or(Error::InsufficientResources)?
  );

  let new_resources_b = world.ruler(&ruler_b)?.resources();
  assert_eq!(new_resources_b, initial_resources_b + resources_to_send);

  Ok(())
}

#[test]
fn cannot_send_resources_to_self() -> Result<()> {
  let mut world = make_world()?;
  let player = PlayerId::from("Player A");
  spawn_player(&mut world, player.as_str())?;

  let ruler = Ruler::from(player);
  let resources = Resources::splat(1000);

  let result = world.send_resources(&ruler, &ruler, resources);
  assert_matches!(result, Err(Error::ResourceReceiverIsSender(..)));

  Ok(())
}

#[test]
fn cannot_send_resources_if_insufficient() -> Result<()> {
  let mut world = make_world()?;
  let player_a = PlayerId::from("Player A");
  spawn_player(&mut world, player_a.as_str())?;

  let player_b = PlayerId::from("Player B");
  spawn_player(&mut world, player_b.as_str())?;

  let ruler_a = Ruler::from(player_a);
  let ruler_b = Ruler::from(player_b);
  let resources = Resources::splat(u32::MAX);

  let result = world.send_resources(&ruler_a, &ruler_b, resources);
  assert_matches!(result, Err(Error::InsufficientResources));

  Ok(())
}
