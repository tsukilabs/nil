// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::Result;
use crate::military::army::personnel::ArmyPersonnel;
use crate::military::maneuver::{ManeuverKind, ManeuverRequest};
use crate::player::PlayerId;
use crate::tests::{get_first_coord, make_world, spawn_player};

#[test]
#[cfg_attr(miri, ignore)]
fn no_self_harm() -> Result<()> {
  let mut world = make_world()?;

  let player_a = PlayerId::from("Player A");
  spawn_player(&mut world, player_a.as_str())?;

  let player_b = PlayerId::from("Player B");
  spawn_player(&mut world, player_b.as_str())?;

  let coord_a = get_first_coord(&world, &player_a);
  let coord_b = get_first_coord(&world, &player_b);

  let personnel = ArmyPersonnel::builder()
    .heavy_cavalry(1000)
    .build();

  world
    .military
    .spawn(coord_b, &player_a, personnel.clone());

  assert!(
    world
      .military()
      .fold_personnel_at(coord_a)
      .is_empty()
  );

  let request = ManeuverRequest::builder()
    .kind(ManeuverKind::Attack)
    .ruler(&player_a)
    .origin(coord_b)
    .destination(coord_a)
    .personnel(personnel.clone())
    .build();

  let maneuver_id = world.request_maneuver(request)?;

  while world.military.maneuver(maneuver_id).is_ok() {
    world.process_maneuvers()?;
  }

  assert!(
    world
      .military()
      .fold_personnel_at(coord_a)
      .heavy_cavalry()
      .eq(personnel.heavy_cavalry())
  );

  Ok(())
}
