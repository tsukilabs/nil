// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::maneuver::{ManeuverKind, ManeuverRequest};
use crate::military::unit::r#impl::heavy_cavalry::HeavyCavalry;
use crate::player::PlayerId;
use crate::tests::{CONFIG, get_first_coord, make_world, spawn_player};
use std::assert_matches;

#[test]
#[cfg_attr(miri, ignore)]
fn has_enough_personnel_to_maneuver() -> Result<()> {
  let mut world = make_world()?;

  let player_a = PlayerId::from("Player A");
  spawn_player(&mut world, player_a.as_str())?;

  let player_b = PlayerId::from("Player B");
  spawn_player(&mut world, player_b.as_str())?;

  let coord_a = get_first_coord(&world, &player_a);
  let coord_b = get_first_coord(&world, &player_b);

  let request = ManeuverRequest::builder()
    .kind(ManeuverKind::Attack)
    .ruler(&player_a)
    .origin(coord_a)
    .destination(coord_b)
    .personnel(ArmyPersonnel::splat(1000))
    .build();

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
#[cfg_attr(miri, ignore)]
fn slowest_maneuver_squad() -> Result<()> {
  let mut world = make_world()?;

  let player_a = PlayerId::from("Player A");
  spawn_player(&mut world, player_a.as_str())?;

  let player_b = PlayerId::from("Player B");
  spawn_player(&mut world, player_b.as_str())?;

  let coord_a = get_first_coord(&world, &player_a);
  let coord_b = get_first_coord(&world, &player_b);

  let spawned = ArmyPersonnel::splat(1000);
  let sent = ArmyPersonnel::builder()
    .heavy_cavalry(1000)
    .light_cavalry(1000)
    .build();

  world
    .military
    .spawn(coord_a, &player_a, spawned.clone());

  let request = ManeuverRequest::builder()
    .kind(ManeuverKind::Attack)
    .ruler(&player_a)
    .origin(coord_a)
    .destination(coord_b)
    .personnel(sent.clone())
    .build();

  let maneuver_id = world.request_maneuver(request)?;
  let maneuver = world.military.maneuver(maneuver_id)?;
  let army = world.military.army(maneuver.army())?;
  let slowest = army.slowest_squad(&CONFIG).unwrap();

  assert_eq!(slowest, sent.slowest_squad(&CONFIG).unwrap());
  assert_eq!(slowest.speed(&CONFIG), HeavyCavalry::STATS.base_speed());
  assert_ne!(slowest, spawned.slowest_squad(&CONFIG).unwrap());

  Ok(())
}
