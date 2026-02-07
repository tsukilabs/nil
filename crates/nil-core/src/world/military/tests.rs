// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::continent::Coord;
use crate::error::{Error, Result};
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::maneuver::{ManeuverKind, ManeuverRequest};
use crate::military::unit::heavy_cavalry::HeavyCavalry;
use crate::player::{PlayerId, PlayerOptions};
use crate::world::{World, WorldOptions};
use std::assert_matches;

#[test]
fn has_enough_personnel_to_maneuver() -> Result<()> {
  let mut world = make_world()?;

  let player_a = PlayerId::from("Player A");
  spawn(&mut world, &player_a)?;

  let player_b = PlayerId::from("Player B");
  spawn(&mut world, &player_b)?;

  let coord_a = get_coord(&world, &player_a);
  let coord_b = get_coord(&world, &player_b);

  let request = ManeuverRequest {
    kind: ManeuverKind::Attack,
    origin: coord_a,
    destination: coord_b,
    personnel: ArmyPersonnel::splat(1000),
  };

  assert_matches!(
    world.request_maneuver(request.clone()),
    Err(Error::InsufficientUnits)
  );

  world
    .military
    .spawn(coord_a, player_a, ArmyPersonnel::splat(1000));

  assert!(world.request_maneuver(request).is_ok());

  Ok(())
}

#[test]
fn slowest_maneuver_squad() -> Result<()> {
  let mut world = make_world()?;

  let player_a = PlayerId::from("Player A");
  spawn(&mut world, &player_a)?;

  let player_b = PlayerId::from("Player B");
  spawn(&mut world, &player_b)?;

  let coord_a = get_coord(&world, &player_a);
  let coord_b = get_coord(&world, &player_b);

  let spawned = ArmyPersonnel::splat(1000);
  let sent = ArmyPersonnel::builder()
    .heavy_cavalry(1000)
    .light_cavalry(1000)
    .build();

  world
    .military
    .spawn(coord_a, player_a, spawned.clone());

  let request = ManeuverRequest {
    kind: ManeuverKind::Attack,
    origin: coord_a,
    destination: coord_b,
    personnel: sent.clone(),
  };

  let maneuver_id = world.request_maneuver(request)?;
  let maneuver = world.military.maneuver(maneuver_id)?;
  let army = world.military.army(maneuver.army())?;
  let slowest = army.slowest_squad().unwrap();

  assert_eq!(slowest, sent.slowest_squad().unwrap());
  assert_eq!(slowest.speed(), HeavyCavalry::STATS.speed());
  assert_ne!(slowest, spawned.slowest_squad().unwrap());

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

fn get_coord(world: &World, id: &PlayerId) -> Coord {
  world
    .continent
    .coords_of(id.clone())
    .next()
    .unwrap()
}
